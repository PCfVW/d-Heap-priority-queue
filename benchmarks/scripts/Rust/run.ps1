# run.ps1 — Phase 3 Rust benchmark runner
# Per benchmarks/methodology.md: 3-pass capture (wall-time, stats, RSS).
#
# Usage:
#   pwsh -File benchmarks/scripts/Rust/run.ps1               # full sweep (51 files)
#   pwsh -File benchmarks/scripts/Rust/run.ps1 -SmokeTest    # one cell only (huge_dense × d=8)
#   pwsh -File benchmarks/scripts/Rust/run.ps1 -SkipBuild    # reuse existing target/release/dijkstra-example.exe

#Requires -Version 7.0

[CmdletBinding()]
param(
    [switch]$SmokeTest,
    [switch]$SkipBuild
)

$ErrorActionPreference = 'Stop'

# --- Path resolution ---
$repoRoot       = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path
$rustExampleDir = Join-Path $repoRoot 'examples\dijkstra\Rust'
$binary         = Join-Path $rustExampleDir 'target\release\dijkstra-example.exe'
$envFile        = Join-Path $PSScriptRoot '..\env-Rust.json'
$resultsDir     = Join-Path $repoRoot 'benchmarks\results\Rust'

if (-not (Test-Path $envFile)) {
    throw "env.json not found at $envFile. Run benchmarks/scripts/env.ps1 first."
}
$envFile = (Resolve-Path $envFile).Path

New-Item -ItemType Directory -Path $resultsDir -Force | Out-Null

# --- Build ---
if (-not $SkipBuild) {
    Write-Host "Building Rust binary (release)..."
    Push-Location $rustExampleDir
    try {
        cargo build --release
        if ($LASTEXITCODE -ne 0) { throw "cargo build failed" }
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
    # Binary self-reports peak RSS via GetProcessMemoryInfo.
    # External observation (Start-Process -Wait then PeakWorkingSet64) doesn't
    # work on Windows for fast processes — the OS drops the data on exit.
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
