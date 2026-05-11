//! Non-Windows peak-RSS stub for Phase 3 benchmark harness.
//! Add /proc/self/status (Linux) or task_info (macOS) when Phase 3 ports off Windows.

pub fn peakRssKb() ?u64 {
    return null;
}
