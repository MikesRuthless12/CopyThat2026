//! Phase 47 polish — per-OS disk-busy sampling for the "why is this slow?"
//! diagnostics ([`copythat_diag::DiagSnapshot`]'s `src_disk_busy_pct` /
//! `dst_disk_busy_pct`).
//!
//! [`DiskBusySampler`] is a stateful, cross-platform sampler the
//! diagnostics task ticks once per second. Each [`tick`](DiskBusySampler::tick)
//! refreshes a per-volume "% busy" reading; [`busy_pct_for_path`] then
//! attributes a reading to a copy's source / destination by the volume the
//! path lives on.
//!
//! - **Windows:** the `\LogicalDisk(*)\% Disk Time` PDH counter (no
//!   elevation required), matched to a path by its drive letter.
//! - **Linux:** `/proc/diskstats` field 10 (`io_ticks`, ms the device had
//!   I/O in flight) differenced across the tick interval, matched to a
//!   path's backing block device via `/proc/self/mountinfo`.
//! - **macOS / other:** unsupported — [`DiskBusySampler::new`] returns
//!   `None`, so the diagnostics task simply leaves the disk fields unset
//!   (the classifier already treats them as "unsampled"). `iostat` exists
//!   but spawning it every second would perturb the very throughput we're
//!   measuring.
//!
//! Lives here (the one crate where `unsafe` is allowed) because the
//! Windows reading is a raw PDH FFI call, mirroring how the Phase 31b
//! WinRT network probe sits in [`crate::network`].
//!
//! [`busy_pct_for_path`]: DiskBusySampler::busy_pct_for_path

use std::path::Path;

/// A stateful per-volume disk-busy sampler. Create once, [`tick`] each
/// second, then read [`busy_pct_for_path`] / [`peak_pct`].
///
/// [`tick`]: DiskBusySampler::tick
/// [`busy_pct_for_path`]: DiskBusySampler::busy_pct_for_path
/// [`peak_pct`]: DiskBusySampler::peak_pct
pub struct DiskBusySampler {
    inner: imp::Inner,
}

impl DiskBusySampler {
    /// Build a sampler, or `None` on an unsupported platform (macOS / other)
    /// or when the OS performance source can't be opened.
    pub fn new() -> Option<Self> {
        imp::Inner::new().map(|inner| Self { inner })
    }

    /// Refresh the per-volume busy readings for the interval since the
    /// previous tick. The first tick after [`new`](Self::new) primes the
    /// baseline and yields no readings.
    pub fn tick(&mut self) {
        self.inner.tick();
    }

    /// Busy percent (`0.0..=100.0`) of the volume backing `path`, or `None`
    /// when the path isn't on a sampled local volume (e.g. a UNC / network
    /// path, which the classifier attributes via its network signal
    /// instead) or no reading is available yet.
    pub fn busy_pct_for_path(&self, path: &Path) -> Option<f32> {
        self.inner.busy_pct_for_path(path)
    }

    /// The busiest local volume's percent this tick — a coarse "is any
    /// disk saturated" fallback when a path can't be attributed.
    pub fn peak_pct(&self) -> Option<f32> {
        self.inner.peak_pct()
    }
}

// ---------------------------------------------------------------------
// Windows — `\LogicalDisk(*)\% Disk Time` via PDH.
// ---------------------------------------------------------------------
#[cfg(target_os = "windows")]
mod imp {
    use std::path::{Component, Path, Prefix};

    use windows_sys::Win32::System::Performance::{
        PDH_FMT_COUNTERVALUE_ITEM_W, PdhAddEnglishCounterW, PdhCloseQuery, PdhCollectQueryData,
        PdhGetFormattedCounterArrayW, PdhOpenQueryW,
    };

    // PDH status / format codes. Defined locally so the exact windows-sys
    // constant surface (which moves between minor versions) isn't a build
    // dependency — these values are stable parts of the PDH ABI.
    const ERROR_OK: u32 = 0;
    const PDH_MORE_DATA: u32 = 0x8000_07D2;
    const PDH_FMT_DOUBLE: u32 = 0x0000_0200;
    /// Don't cap the formatted value at 100 — `% Disk Time` legitimately
    /// exceeds 100 on multi-spindle volumes; we clamp ourselves.
    const PDH_FMT_NOCAP100: u32 = 0x0000_8000;
    const PDH_FMT: u32 = PDH_FMT_DOUBLE | PDH_FMT_NOCAP100;

    pub struct Inner {
        query: isize,
        counter: isize,
        /// (drive letter upper-cased, busy percent) for this tick.
        readings: Vec<(u8, f32)>,
    }

    // SAFETY: the PDH query/counter handles are owned by this struct and
    // only touched behind `&mut self`; PDH handles are process-global and
    // valid to move between threads. The diagnostics task owns the sampler
    // on a single tokio task, so this is conservative.
    unsafe impl Send for Inner {}

    impl Inner {
        pub fn new() -> Option<Self> {
            let path: Vec<u16> = "\\LogicalDisk(*)\\% Disk Time\0".encode_utf16().collect();
            let mut query: isize = 0;
            // SAFETY: standard PDH open; null data source = live data.
            let status = unsafe { PdhOpenQueryW(std::ptr::null(), 0, &mut query) } as u32;
            if status != ERROR_OK {
                return None;
            }
            let mut counter: isize = 0;
            // SAFETY: `path` is a NUL-terminated UTF-16 counter path; the
            // query handle is valid from the successful open above.
            let status =
                unsafe { PdhAddEnglishCounterW(query, path.as_ptr(), 0, &mut counter) } as u32;
            if status != ERROR_OK {
                // SAFETY: closing a successfully-opened query.
                unsafe { PdhCloseQuery(query) };
                return None;
            }
            // Prime the first collect — `% Disk Time` is a rate counter and
            // needs two samples before it yields a value.
            // SAFETY: valid query handle.
            unsafe { PdhCollectQueryData(query) };
            Some(Self {
                query,
                counter,
                readings: Vec::new(),
            })
        }

        pub fn tick(&mut self) {
            // SAFETY: valid query handle owned by self.
            let status = unsafe { PdhCollectQueryData(self.query) } as u32;
            if status != ERROR_OK {
                self.readings.clear();
                return;
            }
            self.readings = self.read_array().unwrap_or_default();
        }

        fn read_array(&self) -> Option<Vec<(u8, f32)>> {
            let mut buf_size: u32 = 0;
            let mut item_count: u32 = 0;
            // First call: ask for the required buffer size (null item buffer
            // with zeroed size/count is the documented "query size" form).
            // SAFETY: outputs the needed byte size + item count.
            let status = unsafe {
                PdhGetFormattedCounterArrayW(
                    self.counter,
                    PDH_FMT,
                    &mut buf_size,
                    &mut item_count,
                    std::ptr::null_mut(),
                )
            } as u32;
            if status != PDH_MORE_DATA || buf_size == 0 || item_count == 0 {
                return None;
            }
            // PDH writes the item array *and* the instance-name strings into
            // one buffer; allocate the exact byte size it asked for, aligned
            // for the item struct.
            let cap = item_count as usize;
            let mut items: Vec<PDH_FMT_COUNTERVALUE_ITEM_W> = Vec::with_capacity(
                (buf_size as usize).div_ceil(size_of::<PDH_FMT_COUNTERVALUE_ITEM_W>()),
            );
            // SAFETY: capacity covers `buf_size` bytes as PDH requires;
            // PDH fills `item_count` items plus trailing wide strings the
            // `szName` pointers reference within the same allocation.
            let status = unsafe {
                PdhGetFormattedCounterArrayW(
                    self.counter,
                    PDH_FMT,
                    &mut buf_size,
                    &mut item_count,
                    items.as_mut_ptr(),
                )
            } as u32;
            if status != ERROR_OK {
                return None;
            }
            let mut out = Vec::with_capacity(cap);
            for i in 0..item_count as usize {
                // SAFETY: PDH populated `item_count` items in the buffer.
                let item = unsafe { &*items.as_ptr().add(i) };
                if item.FmtValue.CStatus != ERROR_OK {
                    continue;
                }
                // SAFETY: szName points to a NUL-terminated UTF-16 string
                // inside the buffer for as long as `items` lives.
                let name = unsafe { wide_to_string(item.szName) };
                // `\LogicalDisk` instance names are "C:", "D:", "_Total",
                // "HarddiskVolumeN". Keep only single-letter drive volumes.
                if let Some(letter) = drive_letter_of_instance(&name) {
                    // SAFETY: we formatted with PDH_FMT_DOUBLE, so the
                    // double arm of the value union is the initialised one.
                    let raw = unsafe { item.FmtValue.Anonymous.doubleValue };
                    let pct = raw.clamp(0.0, 100.0) as f32;
                    out.push((letter, pct));
                }
            }
            Some(out)
        }

        pub fn busy_pct_for_path(&self, path: &Path) -> Option<f32> {
            let letter = drive_letter_of_path(path)?;
            self.readings
                .iter()
                .find(|(l, _)| *l == letter)
                .map(|(_, pct)| *pct)
        }

        pub fn peak_pct(&self) -> Option<f32> {
            self.readings
                .iter()
                .map(|(_, pct)| *pct)
                .fold(None, |acc, p| Some(acc.map_or(p, |a: f32| a.max(p))))
        }
    }

    impl Drop for Inner {
        fn drop(&mut self) {
            // SAFETY: closing the query we opened; PDH closes the child
            // counter handle with it.
            unsafe { PdhCloseQuery(self.query) };
        }
    }

    /// Read a NUL-terminated UTF-16 string PDH wrote into our buffer.
    ///
    /// SAFETY: `ptr` must be null or point to a NUL-terminated UTF-16
    /// string (PDH's `szName`, which lives inside the array buffer for its
    /// lifetime).
    unsafe fn wide_to_string(ptr: *const u16) -> String {
        if ptr.is_null() {
            return String::new();
        }
        let mut len = 0usize;
        // SAFETY: caller guarantees a NUL terminator bounds this walk.
        while unsafe { *ptr.add(len) } != 0 {
            len += 1;
        }
        // SAFETY: `len` u16s are valid and initialised up to the NUL.
        let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
        String::from_utf16_lossy(slice)
    }

    /// "C:" -> `b'C'`; "_Total" / "HarddiskVolume1" -> `None`.
    fn drive_letter_of_instance(name: &str) -> Option<u8> {
        let bytes = name.as_bytes();
        if bytes.len() == 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic() {
            Some(bytes[0].to_ascii_uppercase())
        } else {
            None
        }
    }

    /// Drive letter a path lives on; `None` for UNC / verbatim-UNC
    /// (network) paths, which the classifier handles via its network signal.
    fn drive_letter_of_path(path: &Path) -> Option<u8> {
        match path.components().next() {
            Some(Component::Prefix(prefix)) => match prefix.kind() {
                Prefix::Disk(letter) | Prefix::VerbatimDisk(letter) => {
                    Some(letter.to_ascii_uppercase())
                }
                _ => None,
            },
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------
// Linux — `/proc/diskstats` io_ticks, attributed via `/proc/self/mountinfo`.
// ---------------------------------------------------------------------
#[cfg(target_os = "linux")]
mod imp {
    use std::collections::HashMap;
    use std::path::Path;
    use std::time::Instant;

    pub struct Inner {
        /// device name -> io_ticks (ms with I/O in flight), previous tick.
        prev: HashMap<String, u64>,
        prev_at: Instant,
        /// device name -> busy percent this tick.
        readings: HashMap<String, f32>,
    }

    impl Inner {
        pub fn new() -> Option<Self> {
            // Require the stats file to exist before claiming support.
            let prev = read_diskstats()?;
            Some(Self {
                prev,
                prev_at: Instant::now(),
                readings: HashMap::new(),
            })
        }

        pub fn tick(&mut self) {
            let now = Instant::now();
            let elapsed_ms = now.duration_since(self.prev_at).as_secs_f64() * 1000.0;
            let Some(cur) = read_diskstats() else {
                self.readings.clear();
                return;
            };
            if elapsed_ms <= 0.0 {
                return;
            }
            self.readings.clear();
            for (dev, ticks) in &cur {
                if let Some(prev) = self.prev.get(dev) {
                    let busy_ms = ticks.saturating_sub(*prev) as f64;
                    let pct = (busy_ms / elapsed_ms * 100.0).clamp(0.0, 100.0) as f32;
                    self.readings.insert(dev.clone(), pct);
                }
            }
            self.prev = cur;
            self.prev_at = now;
        }

        pub fn busy_pct_for_path(&self, path: &Path) -> Option<f32> {
            let dev = device_for_path(path)?;
            // A partition (sda1) inherits its whole-disk (sda) utilisation
            // when the partition itself isn't a diskstats row.
            self.readings
                .get(&dev)
                .copied()
                .or_else(|| self.readings.get(&whole_disk(&dev)).copied())
        }

        pub fn peak_pct(&self) -> Option<f32> {
            self.readings
                .values()
                .copied()
                .fold(None, |acc, p| Some(acc.map_or(p, |a: f32| a.max(p))))
        }
    }

    /// Parse `/proc/diskstats` into device -> io_ticks. Columns are
    /// `major minor name` followed by 11 stat fields; `io_ticks` is the
    /// 10th stat field, i.e. whitespace index 12.
    fn read_diskstats() -> Option<HashMap<String, u64>> {
        let content = std::fs::read_to_string("/proc/diskstats").ok()?;
        let mut map = HashMap::new();
        for line in content.lines() {
            let f: Vec<&str> = line.split_whitespace().collect();
            if f.len() < 13 {
                continue;
            }
            let name = f[2].to_string();
            // Skip loop / ram pseudo-devices.
            if name.starts_with("loop") || name.starts_with("ram") {
                continue;
            }
            if let Ok(io_ticks) = f[12].parse::<u64>() {
                map.insert(name, io_ticks);
            }
        }
        Some(map)
    }

    /// Resolve a path to the kernel device name backing its mount via
    /// `/proc/self/mountinfo` (longest mount-point prefix wins).
    fn device_for_path(path: &Path) -> Option<String> {
        let target = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let mounts = std::fs::read_to_string("/proc/self/mountinfo").ok()?;
        let mut best: Option<(usize, String)> = None;
        for line in mounts.lines() {
            // mountinfo: ... <mount_point> ... - <fstype> <source> ...
            let fields: Vec<&str> = line.split_whitespace().collect();
            let Some(dash) = fields.iter().position(|&f| f == "-") else {
                continue;
            };
            let (Some(mount_point), Some(source)) = (fields.get(4), fields.get(dash + 2)) else {
                continue;
            };
            // mountinfo octal-escapes space/tab/newline/backslash; decode
            // before path-matching so a mount point containing a space still
            // matches the canonicalized target.
            let mount_point = unescape_mountinfo(mount_point);
            if target.starts_with(&mount_point) {
                let len = mount_point.len();
                if best.as_ref().is_none_or(|(b, _)| len > *b) {
                    // /dev/sda1 -> sda1
                    let source = unescape_mountinfo(source);
                    let dev = source.rsplit('/').next().unwrap_or(&source).to_string();
                    best = Some((len, dev));
                }
            }
        }
        best.map(|(_, dev)| dev)
    }

    /// Decode `/proc/self/mountinfo` octal escapes (`\040` space, `\011`
    /// tab, `\012` newline, `\134` backslash) back to literal bytes. The
    /// escaped characters are all ASCII, so surrounding UTF-8 is preserved.
    fn unescape_mountinfo(s: &str) -> String {
        if !s.contains('\\') {
            return s.to_string();
        }
        let bytes = s.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'\\'
                && i + 4 <= bytes.len()
                && bytes[i + 1..i + 4]
                    .iter()
                    .all(|b| (b'0'..=b'7').contains(b))
            {
                if let Some(b) = std::str::from_utf8(&bytes[i + 1..i + 4])
                    .ok()
                    .and_then(|o| u8::from_str_radix(o, 8).ok())
                {
                    out.push(b);
                    i += 4;
                    continue;
                }
            }
            out.push(bytes[i]);
            i += 1;
        }
        String::from_utf8(out).unwrap_or_else(|_| s.to_string())
    }

    /// Whole-disk name for a partition: "nvme0n1p2" / "mmcblk0p1" -> base;
    /// "sda1" -> "sda". Idempotent on whole-disk names ("nvme0n1" stays
    /// "nvme0n1", "sda" stays "sda").
    fn whole_disk(dev: &str) -> String {
        // Device classes whose base name ends in a digit use a `pN` partition
        // suffix (nvme0n1 -> nvme0n1p2, mmcblk0 -> mmcblk0p1); the rest append
        // the partition number directly (sda -> sda1). For the former, strip
        // only a real `pN` (base ends in a digit, so the `p` in "loop0" is not
        // mistaken for a separator) and otherwise return the name unchanged —
        // never trimming the base digit.
        const P_SEP_CLASSES: &[&str] = &["nvme", "mmcblk", "loop", "nbd", "md", "dm-"];
        if P_SEP_CLASSES.iter().any(|c| dev.starts_with(c)) {
            if let Some(idx) = dev.rfind('p') {
                let (base, suffix) = dev.split_at(idx);
                let part = &suffix[1..];
                if !part.is_empty()
                    && part.bytes().all(|b| b.is_ascii_digit())
                    && base.bytes().next_back().is_some_and(|b| b.is_ascii_digit())
                {
                    return base.to_string();
                }
            }
            return dev.to_string();
        }
        dev.trim_end_matches(|c: char| c.is_ascii_digit())
            .to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::{unescape_mountinfo, whole_disk};

        #[test]
        fn whole_disk_handles_pn_and_plain_partitions() {
            assert_eq!(whole_disk("nvme0n1p2"), "nvme0n1");
            assert_eq!(whole_disk("mmcblk0p1"), "mmcblk0");
            assert_eq!(whole_disk("sda1"), "sda");
            assert_eq!(whole_disk("sda"), "sda");
            assert_eq!(whole_disk("nvme0n1"), "nvme0n1");
        }

        #[test]
        fn unescape_decodes_octal() {
            assert_eq!(unescape_mountinfo("/mnt/My\\040Disk"), "/mnt/My Disk");
            assert_eq!(unescape_mountinfo("/home/user"), "/home/user");
            assert_eq!(unescape_mountinfo("/a\\134b"), "/a\\b");
        }
    }
}

// ---------------------------------------------------------------------
// macOS / other — unsupported (no per-second-cheap busy source).
// ---------------------------------------------------------------------
#[cfg(not(any(target_os = "windows", target_os = "linux")))]
mod imp {
    use std::path::Path;

    pub struct Inner;

    impl Inner {
        pub fn new() -> Option<Self> {
            None
        }
        pub fn tick(&mut self) {}
        pub fn busy_pct_for_path(&self, _path: &Path) -> Option<f32> {
            None
        }
        pub fn peak_pct(&self) -> Option<f32> {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsupported_or_constructs() {
        // On Windows/Linux this should construct (CI hosts have the perf
        // source); on macOS/other it returns None. Either way: no panic,
        // and tick()/reads are safe to call.
        if let Some(mut s) = DiskBusySampler::new() {
            s.tick();
            // peak is either None or a sane percentage.
            if let Some(p) = s.peak_pct() {
                assert!((0.0..=100.0).contains(&p), "peak out of range: {p}");
            }
        }
    }
}
