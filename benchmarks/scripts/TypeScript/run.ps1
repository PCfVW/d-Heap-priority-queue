# run.ps1 — Phase 3 TypeScript benchmark runner
# Per benchmarks/methodology.md: 3-pass capture (wall-time, stats, RSS).
#
# Usage:
#   pwsh -File benchmarks/scripts/TypeScript/run.ps1             # full sweep (51 files)
#   pwsh -File benchmarks/scripts/TypeScript/run.ps1 -SmokeTest  # one cell only (huge_dense × d=8)
#   pwsh -File benchmarks/scripts/TypeScript/run.ps1 -SkipBuild  # reuse existing dist/index.js

#Requires -Version 7.0

[CmdletBinding()]
param(
    [switch]$SmokeTest,
    [switch]$SkipBuild
)

$ErrorActionPreference = 'Stop'

# --- Path resolution ---
$repoRoot     = (Resolve-Path (Join-Path $PSScriptRoot '..\..\..')).Path
$tsExampleDir = Join-Path $repoRoot 'examples\dijkstra\TypeScript'
$entry        = Join-Path $tsExampleDir 'dist\index.js'
$envFile      = Join-Path $PSScriptRoot '..\env-TypeScript.json'
$resultsDir   = Join-Path $repoRoot 'benchmarks\results\TypeScript'

if (-not (Test-Path $envFile)) {
    throw "env-TypeScript.json not found at $envFile. Run benchmarks/scripts/env.ps1 -Language TypeScript first."
}
$envFile = (Resolve-Path $envFile).Path

New-Item -ItemType Directory -Path $resultsDir -Force | Out-Null

# --- Build ---
if (-not $SkipBuild) {
    Write-Host "Building TypeScript example (tsc)..."
    Push-Location $tsExampleDir
    try {
        npm run build
        if ($LASTEXITCODE -ne 0) { throw "npm run build failed" }
    } finally {
        Pop-Location
    }
}
if (-not (Test-Path $entry)) {
    throw "Entry not found at $entry. Run without -SkipBuild."
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
        & node $entry --graph=$g --arity=$d --warmup=2 --repetitions=10 --json --env-file=$envFile |
            Set-Content -Path $out -Encoding utf8NoBOM
        if ($LASTEXITCODE -ne 0) { throw "node failed on $g d=$d (wall-time)" }
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
        & node $entry --graph=$g --arity=$d --stats --json |
            Set-Content -Path $out -Encoding utf8NoBOM
        if ($LASTEXITCODE -ne 0) { throw "node failed on $g d=$d (stats)" }
    }
}

# --- Pass 3: peak RSS (huge_dense only) ---
Write-Host "`n=== Pass 3: peak RSS (huge_dense only) ==="
$rssGraph   = 'huge_dense'
$rssArities = if ($SmokeTest) { @(8) } else { @(2, 4, 8) }
foreach ($d in $rssArities) {
    $out = Join-Path $resultsDir "${rssGraph}_d${d}.rss.json"
    Write-Host "  $rssGraph d=$d -> $(Split-Path $out -Leaf)"
    & node $entry --graph=$rssGraph --arity=$d --report-rss |
        Set-Content -Path $out -Encoding utf8NoBOM
    if ($LASTEXITCODE -ne 0) { throw "node failed on $rssGraph d=$d (RSS)" }
}

# --- Summary ---
Write-Host "`n=== Done ==="
$jsonlCount = (Get-ChildItem -Path $resultsDir -Filter '*.jsonl').Count
$statsCount = (Get-ChildItem -Path $resultsDir -Filter '*.stats.json').Count
$rssCount   = (Get-ChildItem -Path $resultsDir -Filter '*.rss.json').Count
Write-Host "  Wall-time files: $jsonlCount"
Write-Host "  Stats files:     $statsCount"
Write-Host "  RSS files:       $rssCount"
