//! Windows peak-RSS reader for Phase 3 benchmark harness.
//! Uses Win32 GetProcessMemoryInfo via manual extern (Zig 0.15.2 stdlib
//! does not expose psapi). Mirrors the Win32 path the Rust + Go harnesses take.

const std = @import("std");

const PROCESS_MEMORY_COUNTERS = extern struct {
    cb: u32,
    PageFaultCount: u32,
    PeakWorkingSetSize: usize,
    WorkingSetSize: usize,
    QuotaPeakPagedPoolUsage: usize,
    QuotaPagedPoolUsage: usize,
    QuotaPeakNonPagedPoolUsage: usize,
    QuotaNonPagedPoolUsage: usize,
    PagefileUsage: usize,
    PeakPagefileUsage: usize,
};

extern "psapi" fn GetProcessMemoryInfo(
    Process: std.os.windows.HANDLE,
    ppsmemCounters: *PROCESS_MEMORY_COUNTERS,
    cb: u32,
) callconv(.winapi) std.os.windows.BOOL;

extern "kernel32" fn GetCurrentProcess() callconv(.winapi) std.os.windows.HANDLE;

pub fn peakRssKb() ?u64 {
    var info: PROCESS_MEMORY_COUNTERS = undefined;
    info.cb = @sizeOf(PROCESS_MEMORY_COUNTERS);
    if (GetProcessMemoryInfo(GetCurrentProcess(), &info, info.cb) == 0) return null;
    return @as(u64, info.PeakWorkingSetSize) / 1024;
}
