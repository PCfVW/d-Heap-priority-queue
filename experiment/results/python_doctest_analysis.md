# Python Doctest Experiment Results

## Configuration
- Model: claude-sonnet-4-20250514
- Prompt doctests: 50
- Output doctests: 50
- Amplification ratio: 1.00x

## Method-by-Method Analysis
- __init__: 2
- insert: 4
- pop: 7
- front: 5
- increase_priority: 6
- decrease_priority: 6
- contains: 4
- __len__: 4
- is_empty: 4
- __eq__: 2

## Interpretation
PRESERVATION: Model maintains all doctests (100% scaffolding like Rust/Zig).

## Raw Metrics
- Input tokens: 1567
- Output tokens: 2504
- Response time: 29.26s
