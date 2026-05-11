//go:build !windows

package main

// peakRssKb is a no-op stub on non-Windows platforms.
// Add /proc/self/status (Linux) or task_info (macOS) when Phase 3 ports off Windows.
func peakRssKb() (uint64, error) {
	return 0, nil
}
