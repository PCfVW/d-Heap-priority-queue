# Rust Signal Strength Experiment: No Module Wrapper

## Configuration
- Model: claude-sonnet-4-20250514
- Prompt structure: TOP-LEVEL #[test] functions (no mod wrapper)
- Tests in prompt: 20

## Results
- Tests in output: 20
- Ratio: 1.00x
- Has mod tests wrapper in output: false
- Has #[cfg(test)] in output: false

## Interpretation
PRESERVATION detected - #[test] without mod wrapper does NOT amplify

## Comparison with Original Rust Test-Guided
- Original: 6 prompt tests -> 22 output tests (3.67x AMPLIFICATION)
- Original structure: #[cfg(test)] mod tests { use super::*; ... }
- This experiment: 20 prompt tests -> 20 output tests (1.00x)

## Conclusion
The module wrapper IS the amplification trigger. #[test] alone only preserves.

## Raw Metrics
- Input tokens: 3237
- Output tokens: 4095
- Response time: 39.09s
