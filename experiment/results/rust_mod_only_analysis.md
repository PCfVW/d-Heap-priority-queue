# Rust Signal Strength Experiment: Mod Only (No cfg)

## Configuration
- Model: claude-sonnet-4-20250514
- Prompt structure: mod tests { use super::*; ... } WITHOUT #[cfg(test)]
- Tests in prompt: 20

## Results
- Tests in output: 20
- Ratio: 1.00x
- Has mod tests wrapper in output: true
- Has #[cfg(test)] in output: true

## Interpretation
PRESERVATION detected - mod tests without #[cfg(test)] does NOT amplify

## Comparison
| Experiment | Prompt Structure | Tests In | Tests Out | Ratio | Result |
|------------|------------------|----------|-----------|-------|--------|
| Original | #[cfg(test)] mod tests | 6 | 22 | 3.67x | AMPLIFICATION |
| No Module | Top-level #[test] | 20 | 20 | 1.00x | PRESERVATION |
| Mod Only | mod tests (no cfg) | 20 | 20 | 1.00x | PRESERVATION |

## Raw Metrics
- Input tokens: 3298
- Output tokens: 4461
- Response time: 47.91s
