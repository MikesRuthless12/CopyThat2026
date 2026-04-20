//! Best-effort detection of SSD-backed paths.
//!
//! The public [`is_ssd`] is a synchronous probe: it reads sysfs on
//! Linux, shells out to `diskutil` on macOS, and runs a PowerShell
//! one-liner on Windows. All three paths are strictly
//! information-only — the shredder emits a
//! [`ShredEvent::SsdAdvisory`](crate::event::ShredEvent::SsdAdvisory)
//! and proceeds regardless of the answer.
//!
//! Returns:
//! - `Some(true)` — device is an SSD / NVMe (or the platform's
//!   best-guess equivalent).
//! - `Some(false)` — device is spinning media.
//! - `None` — probe failed or the answer is unknown. Callers should
//!   treat this as "no advisory".

use std::path::Path;

/// Probe whether `path` lives on an SSD. See module-level docs for the
/// exact semantics of `None`.
pub fn is_ssd(path: &Path) -> Option<bool> {
    probe(path)
}

// --------------------------------------------------------------------
// Linux: /sys/block/<dev>/queue/rotational is "0" for SSD/NVMe, "1"
// for HDD. We resolve the path's filesystem to a device via `stat`.
// --------------------------------------------------------------------
#[cfg(target_os = "linux")]
fn probe(path: &Path) -> Option<bool> {
    // Resolve to an absolute path; df / findmnt dislike missing targets,
    // but the shredded target is typically still present or its parent
    // is.
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };

    // Ask `findmnt` which device owns the filesystem at this path.
    let out = std::process::Command::new("findmnt")
        .arg("-nro")
        .arg("SOURCE")
        .arg("-T")
        .arg(&probe_target)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let device = String::from_utf8(out.stdout).ok()?.trim().to_string();
    if device.is_empty() {
        return None;
    }

    // Device looks like `/dev/sda1` or `/dev/nvme0n1p3`. Strip the
    // trailing partition number so we can read the parent block
    // device's rotational flag. We walk up sysfs links until we find a
    // queue directory.
    let dev_name = Path::new(&device)
        .file_name()?
        .to_string_lossy()
        .into_owned();
    let parent_dev = strip_partition_suffix(&dev_name);

    let sys_path = Path::new("/sys/block")
        .join(&parent_dev)
        .join("queue/rotational");
    let flag = std::fs::read_to_string(&sys_path).ok()?;
    match flag.trim() {
        "0" => Some(true),
        "1" => Some(false),
        _ => None,
    }
}

#[cfg(target_os = "linux")]
fn strip_partition_suffix(name: &str) -> String {
    // nvme0n1p3 -> nvme0n1 ; sda1 -> sda ; mmcblk0p2 -> mmcblk0
    if name.starts_with("nvme") || name.starts_with("mmcblk") || name.starts_with("loop") {
        // Split at the last "p<digits>" suffix. A real partition
        // suffix always has a digit *immediately before* the `p`
        // (the device index — e.g. `nvme0n1p3`, `mmcblk0p2`); plain
        // `loop0` has its `p` inside the device name itself, so the
        // char before the `p` is `o` and the match must not strip.
        if let Some(pos) = name.rfind('p') {
            let (head, tail) = name.split_at(pos);
            let head_ends_in_digit = head.chars().next_back().is_some_and(|c| c.is_ascii_digit());
            if head_ends_in_digit && tail.len() > 1 && tail[1..].chars().all(|c| c.is_ascii_digit())
            {
                return head.to_string();
            }
        }
        return name.to_string();
    }
    // Generic SCSI / SATA: trim trailing digits.
    let trimmed: String = name
        .chars()
        .rev()
        .skip_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    if trimmed.is_empty() {
        name.to_string()
    } else {
        trimmed
    }
}

// --------------------------------------------------------------------
// macOS: `diskutil info -plist <mount>` exposes `SolidState` as a key.
// Simpler: `diskutil info <mount>` prints a human line
// `   Solid State: Yes` / `No`.
// --------------------------------------------------------------------
#[cfg(target_os = "macos")]
fn probe(path: &Path) -> Option<bool> {
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };

    let out = std::process::Command::new("diskutil")
        .arg("info")
        .arg(&probe_target)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("Solid State:") {
            let v = rest.trim().to_ascii_lowercase();
            return Some(v.starts_with("yes"));
        }
    }
    None
}

// --------------------------------------------------------------------
// Windows: PowerShell `Get-PhysicalDisk | Select MediaType`.
// MediaType values: `SSD`, `HDD`, `Unspecified`.
// We map the volume letter of `path` to a drive letter and look at the
// first matching physical disk.
// --------------------------------------------------------------------
#[cfg(target_os = "windows")]
fn probe(path: &Path) -> Option<bool> {
    let probe_target = if path.exists() {
        path.to_path_buf()
    } else {
        path.parent()?.to_path_buf()
    };
    // Canonicalize to resolve relative paths, then extract `C:` etc.
    let abs = std::fs::canonicalize(&probe_target).ok()?;
    let abs_str = abs.to_string_lossy().into_owned();
    // `\\?\C:\...` or `C:\...`
    let letter = abs_str
        .chars()
        .find(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())?;

    // PowerShell query: find the physical disk backing the drive letter.
    // We use Get-Partition -> Get-PhysicalDisk (PS 5.1+).
    let script = format!(
        "$ErrorActionPreference='SilentlyContinue'; \
         (Get-Partition -DriveLetter '{letter}' | Get-Disk | \
          Get-PhysicalDisk | Select-Object -First 1 -ExpandProperty MediaType)"
    );
    let out = std::process::Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-Command")
        .arg(&script)
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let token = text.trim().to_ascii_lowercase();
    if token.is_empty() {
        return None;
    }
    // Values observed in the wild:
    //   - "SSD"           -> yes
    //   - "HDD"           -> no
    //   - "Unspecified"   -> unknown
    //   - "SCM"           -> storage-class memory, treat as SSD
    match token.as_str() {
        "ssd" | "scm" => Some(true),
        "hdd" => Some(false),
        _ => None,
    }
}

// --------------------------------------------------------------------
// Fallback for other unix flavors: no probe available.
// --------------------------------------------------------------------
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
fn probe(_path: &Path) -> Option<bool> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn partition_suffix_stripping() {
        assert_eq!(strip_partition_suffix("sda1"), "sda");
        assert_eq!(strip_partition_suffix("sda"), "sda");
        assert_eq!(strip_partition_suffix("nvme0n1p3"), "nvme0n1");
        assert_eq!(strip_partition_suffix("nvme0n1"), "nvme0n1");
        assert_eq!(strip_partition_suffix("mmcblk0p2"), "mmcblk0");
        assert_eq!(strip_partition_suffix("loop0"), "loop0");
    }

    #[test]
    fn probe_returns_option() {
        // Not asserting a value — the host CI runners vary. Just verify
        // the function doesn't panic on a real path.
        let _ = is_ssd(Path::new("."));
    }
}
