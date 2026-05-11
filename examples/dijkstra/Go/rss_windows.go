//go:build windows

package main

import (
	"syscall"
	"unsafe"
)

// peakRssKb returns the peak working set size of the current process in KB.
// Uses syscall.LazyDLL because golang.org/x/sys/windows does not expose
// GetProcessMemoryInfo. Mirrors the Win32 path the Rust harness takes.
var (
	psapi                    = syscall.NewLazyDLL("psapi.dll")
	procGetProcessMemoryInfo = psapi.NewProc("GetProcessMemoryInfo")
	kernel32                 = syscall.NewLazyDLL("kernel32.dll")
	procGetCurrentProcess    = kernel32.NewProc("GetCurrentProcess")
)

type processMemoryCounters struct {
	cb                         uint32
	pageFaultCount             uint32
	peakWorkingSetSize         uintptr
	workingSetSize             uintptr
	quotaPeakPagedPoolUsage    uintptr
	quotaPagedPoolUsage        uintptr
	quotaPeakNonPagedPoolUsage uintptr
	quotaNonPagedPoolUsage     uintptr
	pagefileUsage              uintptr
	peakPagefileUsage          uintptr
}

func peakRssKb() (uint64, error) {
	handle, _, _ := procGetCurrentProcess.Call()
	var info processMemoryCounters
	info.cb = uint32(unsafe.Sizeof(info))
	ret, _, callErr := procGetProcessMemoryInfo.Call(
		handle,
		uintptr(unsafe.Pointer(&info)),
		uintptr(info.cb),
	)
	if ret == 0 {
		return 0, callErr
	}
	return uint64(info.peakWorkingSetSize) / 1024, nil
}
