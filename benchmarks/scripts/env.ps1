# env.ps1 — capture machine environment to benchmarks/scripts/env-<Language>.json
# Per benchmarks/methodology.md § "Environment reporting".
#
# Usage:
#   pwsh -File benchmarks/scripts/env.ps1 -Language Rust
#   pwsh -File benchmarks/scripts/env.ps1 -Language Go
#
# Machine info is captured uniformly; `toolchain` and `build_flags` vary per language.
# Output file: env-<Language>.json (one per language; inlined into each wall-time record
# by the corresponding run.ps1 via --env-file).

#Requires -Version 7.0

[CmdletBinding()]
param(
    [Parameter(Mandatory)]
    [ValidateSet('Rust', 'Go', 'TypeScript', 'Zig', 'Cpp')]
    [string]$Language
)

$ErrorActionPreference = 'Stop'

$proc = Get-CimInstance -ClassName Win32_Processor | Select-Object -First 1
$cs   = Get-CimInstance -ClassName Win32_ComputerSystem
$os   = Get-CimInstance -ClassName Win32_OperatingSystem

# Active power plan name (e.g., "High performance")
$powerPlanLine = (powercfg /getactivescheme) -join ' '
if ($powerPlanLine -match '\(([^)]+)\)') {
    $powerPlan = $matches[1]
} else {
    $powerPlan = 'unknown'
}

# Language-specific toolchain + build_flags
switch ($Language) {
    'Rust'       { $toolchain = (rustc --version).Trim();    $buildFlags = '--release' }
    'Go'         { $toolchain = (go version).Trim();         $buildFlags = 'go build' }
    'TypeScript' { $toolchain = (node --version).Trim();     $buildFlags = 'tsc + node dist/index.js' }
    'Zig'        { $toolchain = (zig version).Trim();        $buildFlags = '-Doptimize=ReleaseFast' }
    'Cpp'        {
        # cl prints a localized version banner on stderr; pick the line
        # containing "C/C++" (locale-neutral marker).
        try {
            $clOut = (& cl 2>&1 | Out-String) -split "`r?`n"
            $banner = $clOut | Where-Object { $_ -match 'C/C\+\+' } | Select-Object -First 1
            $toolchain = if ($banner) { $banner.Trim() } else { 'MSVC cl (version banner not found)' }
        } catch {
            $toolchain = 'MSVC cl (not in PATH; run from Developer PowerShell)'
        }
        $buildFlags = 'Release (/O2 /utf-8)'
    }
}

$commit = (git rev-parse --short HEAD).Trim()

# MaxClockSpeed is in MHz; reflects the rated non-turbo max on most CPUs.
# Boost is not exposed via WMI; left for the user to edit if relevant.
$cpuBaseGhz = [math]::Round($proc.MaxClockSpeed / 1000.0, 2)

$record = [ordered]@{
    os             = "$($os.Caption) build $($os.BuildNumber)"
    cpu            = $proc.Name.Trim()
    cpu_base_ghz   = $cpuBaseGhz
    logical_cores  = [int]$cs.NumberOfLogicalProcessors
    ram_gb         = [int]([math]::Round($cs.TotalPhysicalMemory / 1GB))
    turbo          = 'unknown'
    power_plan     = $powerPlan
    toolchain      = $toolchain
    build_flags    = $buildFlags
    commit         = $commit
    date           = (Get-Date -Format 'o')
    host           = 'redacted'
}

$out = Join-Path $PSScriptRoot "env-$Language.json"
# Compact (single-line) JSON: Rust/Go/TS re-encode env via their JSON libs, but
# the Zig harness splices env verbatim into each JSONL record, so the env file
# itself must be one line to preserve the "one record per line" JSONL invariant.
$record | ConvertTo-Json -Depth 4 -Compress | Set-Content -Path $out -Encoding utf8NoBOM

Write-Host "env captured to $out"
Get-Content $out | Write-Host
