# run.ps1 — Phase 3 C++ benchmark runner
# Per benchmarks/methodology.md: 3-pass capture (wall-time, stats, RSS).
#
# Usage:
#   pwsh -File benchmarks/scripts/Cpp/run.ps1             # full sweep (51 files)
#   pwsh -File benchmarks/scripts/Cpp/run.ps1 -SmokeTest  # one cell only (huge_dense × d=8)
#   pwsh -File benchmarks/scripts/Cpp/run.ps1 -SkipBuild  # reuse existing build/Release/dijkstra.exe

#Requires -Version 7.0

[CmdletBinding()]
param(
    [switch]$SmokeTest,
    [switch]$SkipBuild
)

$ErrorActionPreference = 'Stop'

# --- Resolve cmake (not always on PATH outside a Developer PowerShell) ---
$cmake = Get-Command cmake -ErrorAction SilentlyContinue
if (-not $cmake) {
    $candidates = @(
        "$env:ProgramFiles\Microsoft Visual Studio\18\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
        "$env:ProgramFiles\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
        "$env:ProgramFiles\Microsoft Visual Studio\2022\Professional\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
        "$env:ProgramFiles\Microsoft Visual Studio\2022\Enterprise\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe",
        "$env:ProgramFiles\CMake\bin\cmake.exe"
    )
    $found = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
    if ($found) {
        $cmakeExe = $found
    } else {
        throw "cmake.exe not found. Install CMake or run from a Developer PowerShell."
    }
} else {
    $cmakeExe = $cmake.Source
}

# --- Path resolution ---
$repoRoot      = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path
$cppExampleDir = Join-Path $repoRoot 'examples\dijkstra\Cpp'
$binary        = Join-Path $cppExampleDir 'build\Release\dijkstra.exe'
$envFile       = Join-Path $PSScriptRoot '..\env-Cpp.json'
$resultsDir    = Join-Path $repoRoot 'benchmarks\results\Cpp'

if (-not (Test-Path $envFile)) {
    throw "env-Cpp.json not found at $envFile. Run benchmarks/scripts/env.ps1 -Language Cpp first."
}
$envFile = (Resolve-Path $envFile).Path

New-Item -ItemType Directory -Path $resultsDir -Force | Out-Null

# --- Build ---
if (-not $SkipBuild) {
    Write-Host "Building C++ binary (Release)..."
    Push-Location $cppExampleDir
    try {
        & $cmakeExe -B build
        if ($LASTEXITCODE -ne 0) { throw "cmake configure failed" }
        & $cmakeExe --build build --config Release
        if ($LASTEXITCODE -ne 0) { throw "cmake build failed" }
    } finally {
        Pop-Location
    }
}
if (-not (Test-Path $binary)) {
    throw "Binary not found at $binary. Run without -SkipBuild."
}

# --- Matrix ---
$graphs  = @('small', 'medium_sparse', 'medium_dense', 'medium_grid',
             'large_sparse', 'large_dense', 'large_grid', 'huge_dense')
$arities = @(2, 4, 8)

if ($SmokeTest) {
    $graphs  = @('huge_dense')
    $arities = @(8)
}

# --- Pass 1: wall-time ---
Write-Host "`n=== Pass 1: wall-time (warmup=2, reps=10) ==="
$cells = $graphs.Count * $arities.Count
$cell = 0
foreach ($g in $graphs) {
    foreach ($d in $arities) {
        $cell++
        $out = Join-Path $resultsDir "${g}_d${d}.jsonl"
        Write-Host ("  [{0}/{1}] {2} d={3} -> {4}" -f $cell, $cells, $g, $d, (Split-Path $out -Leaf))
        & $binary --graph=$g --arity=$d --warmup=2 --repetitions=10 --json --env-file=$envFile |
            Set-Content -Path $out -Encoding utf8NoBOM
        if ($LASTEXITCODE -ne 0) { throw "binary failed on $g d=$d (wall-time)" }
    }
}

# --- Pass 2: comparison-count stats ---
Write-Host "`n=== Pass 2: comparison-count stats ==="
$cell = 0
foreach ($g in $graphs) {
    foreach ($d in $arities) {
        $cell++
        $out = Join-Path $resultsDir "${g}_d${d}.stats.json"
        Write-Host ("  [{0}/{1}] {2} d={3} -> {4}" -f $cell, $cells, $g, $d, (Split-Path $out -Leaf))
        & $binary --graph=$g --arity=$d --stats --json |
            Set-Content -Path $out -Encoding utf8NoBOM
        if ($LASTEXITCODE -ne 0) { throw "binary failed on $g d=$d (stats)" }
    }
}

# --- Pass 3: peak RSS (huge_dense only) ---
Write-Host "`n=== Pass 3: peak RSS (huge_dense only) ==="
$rssGraph   = 'huge_dense'
$rssArities = if ($SmokeTest) { @(8) } else { @(2, 4, 8) }
foreach ($d in $rssArities) {
    $out = Join-Path $resultsDir "${rssGraph}_d${d}.rss.json"
    Write-Host "  $rssGraph d=$d -> $(Split-Path $out -Leaf)"
    & $binary --graph=$rssGraph --arity=$d --report-rss |
        Set-Content -Path $out -Encoding utf8NoBOM
    if ($LASTEXITCODE -ne 0) { throw "binary failed on $rssGraph d=$d (RSS)" }
}

# --- Summary ---
Write-Host "`n=== Done ==="
$jsonlCount = (Get-ChildItem -Path $resultsDir -Filter '*.jsonl').Count
$statsCount = (Get-ChildItem -Path $resultsDir -Filter '*.stats.json').Count
$rssCount   = (Get-ChildItem -Path $resultsDir -Filter '*.rss.json').Count
Write-Host "  Wall-time files: $jsonlCount"
Write-Host "  Stats files:     $statsCount"
Write-Host "  RSS files:       $rssCount"
