# Advent of Code

This repository contains solutions for [Advent of Code](https://adventofcode.com/). All should be correct, none are likely to be particularly fast, many will amuse a more experienced rustacean. Enjoy?

## Structure

```
aoc/
├── 2024/       # 2024 solutions in Rust
│   ├── src/
│   │   ├── days/       # Solutions for each day
│   ├── input/          # Puzzle inputs (01.txt, 02.txt, etc.)
│   └── input_test/     # Test inputs
├── 2025/       # 2025 solutions (to be added)
└── ...
```

## Running Solutions

To run any solution, use the following command from the repository root:

```bash
cargo run -p year2024 <day> [test]
```

### Examples

Run day 1 with real input:
```bash
cargo run -p year2024 1
```

Run day 1 with test input:
```bash
cargo run -p year2024 1 test
```

Run day 5 from 2025 (when available):
```bash
cargo run -p year2025 5
```

## Adding a New Year

To add solutions for a new year:

1. Create a new directory: `YYYY/`
2. Copy the structure from an existing year
3. Add the new year to the workspace members in the root `Cargo.toml`
4. Place input files in `YYYY/input/` and `YYYY/input_test/`
