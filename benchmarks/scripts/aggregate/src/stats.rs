//! Median / IQR / min / max over the N=10 timed repetitions per cell, per
//! `benchmarks/methodology.md`'s "Reported summary statistics" section.
//! Mean is deliberately not computed — see methodology.md for why.

/// Summary statistics for one cell's 10 `wall_time_us` samples.
pub struct CellStats {
    pub median: f64,
    /// Q3 − Q1.
    pub iqr: f64,
    pub min: f64,
    pub max: f64,
}

/// Computes [`CellStats`] from an unsorted sample slice.
///
/// Median and quartiles use the exclusive-median method (the sample is split
/// into two halves around the median, excluding the median itself for an odd
/// count; Q1/Q3 are the median of each half). This is exact and unambiguous
/// for the protocol's fixed N=10, and degrades sanely for the smaller sample
/// counts a `-SmokeTest` or partial run might produce.
///
/// # Panics
/// Panics if `samples` is empty — every cell in the benchmark matrix always
/// has at least one recorded repetition; an empty slice indicates a
/// collection bug upstream, not a data-quality issue to paper over.
#[must_use]
pub fn compute(samples: &[f64]) -> CellStats {
    assert!(!samples.is_empty(), "compute() called with zero samples");

    let mut sorted = samples.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).expect("wall_time_us is never NaN"));
    let n = sorted.len();

    let median = median_of(&sorted);
    let (lower, upper) = if n.is_multiple_of(2) {
        (&sorted[..n / 2], &sorted[n / 2..])
    } else {
        (&sorted[..n / 2], &sorted[n / 2 + 1..])
    };
    let q1 = median_of(lower);
    let q3 = median_of(upper);

    CellStats {
        median,
        iqr: q3 - q1,
        min: sorted[0],
        max: sorted[n - 1],
    }
}

/// Median of an already-sorted, non-empty slice.
fn median_of(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n.is_multiple_of(2) {
        f64::midpoint(sorted[n / 2 - 1], sorted[n / 2])
    } else {
        sorted[n / 2]
    }
}
