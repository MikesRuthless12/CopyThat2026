//! Shared Linux/BSD mount-table helper used by the btrfs / zfs
//! backends. Kept in its own module so both per-backend files stay
//! free of mountinfo parsing details.

use std::path::Path;

/// Return the filesystem type of the longest mount-point prefix of
/// `path`. Reads `/proc/self/mountinfo`; returns `None` on non-Linux
/// procfs-less systems (the zfs backend's FreeBSD path calls `zfs
/// list` directly instead).
#[cfg(target_os = "linux")]
pub(super) fn fs_type(path: &Path) -> Option<String> {
    let data = std::fs::read_to_string("/proc/self/mountinfo").ok()?;
    let canon = path.canonicalize().ok()?;
    let mut best: Option<(usize, String)> = None;
    for line in data.lines() {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 5 {
            continue;
        }
        let mount_point = Path::new(fields[4]);
        let mut saw_dash = false;
        let mut fs = None;
        for p in &fields[5..] {
            if *p == "-" {
                saw_dash = true;
                continue;
            }
            if saw_dash {
                fs = Some(*p);
                break;
            }
        }
        let Some(fs) = fs else {
            continue;
        };
        if canon.starts_with(mount_point) {
            let score = mount_point.as_os_str().len();
            if best.as_ref().map(|(s, _)| score > *s).unwrap_or(true) {
                best = Some((score, fs.to_string()));
            }
        }
    }
    best.map(|(_, s)| s)
}

#[cfg(not(target_os = "linux"))]
pub(super) fn fs_type(_path: &Path) -> Option<String> {
    None
}
