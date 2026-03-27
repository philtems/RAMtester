# RAMtester

A comprehensive and efficient RAM memory tester written in Rust.

## Description

RAMtester is a memory testing tool that verifies RAM integrity by executing a series of algorithmic tests. It allocates a memory buffer, splits it into two halves, and compares the data to detect potential errors.

## Features

- **16 different memory tests** including: Standard Fill Test, Random Value, Compare XOR, Compare SUB, Compare MUL, Compare DIV, Compare OR, Compare AND, Sequential Increment, Solid Bits, Block Sequential, Checkerboard, Bit Spread, Bit Flip, Walking Ones, Walking Zeroes
- **Execution modes**: Normal mode (tests 1-10 and 12-16), Ultra mode (all 16 tests), Single test mode
- **Color-coded output** with real-time progress display

## Installation

**Prerequisites:** Rust (version 1.70 or higher). For Windows compilation: `rustup target add x86_64-pc-windows-gnu`

**Build:** `cargo build --release` for Linux, or `cargo build --target x86_64-pc-windows-gnu --release` for Windows

The executable is located at `target/release/ramtester` (or `target/x86_64-pc-windows-gnu/release/ramtester.exe` for Windows).

## Usage

**Syntax:** `ramtester <size> [options]`

**Memory size:**
- `512M`: 512 Megabytes
- `2G`: 2 Gigabytes
- `MAX`: Use 90% of available memory

**Options:**
- `--test <N>`: Run only test number N
- `--ultra`: Ultra mode: run all 16 tests

**Examples:**
- `ramtester 512M` — Normal mode with 512 MB
- `ramtester 1G --ultra` — Ultra mode with 1 GB
- `ramtester 2G --test 5 --loop` — Test #5 in loop mode
- `ramtester MAX --ultra` — Use all available memory in ultra mode

## Test List

1. Standard Fill Test
2. Random Value
3. Compare XOR
4. Compare SUB
5. Compare MUL
6. Compare DIV
7. Compare OR
8. Compare AND
9. Sequential Increment
10. Solid Bits
11. Block Sequential
12. Checkerboard
13. Bit Spread
14. Bit Flip
15. Walking Ones
16. Walking Zeroes

*Note: Normal mode excludes test 11 (Block Sequential) due to its slower execution time.*

## Behavior

Each test allocates a buffer of the requested size and splits it into two halves. Both halves are filled with the same pattern, then compared to detect differences. Errors are displayed with their memory address, and an error map is available for tests with failures.

Press `Ctrl+C` to interrupt execution gracefully; a summary of completed tests is displayed upon exit.


## Author

Philippe TEMESI — https://www.tems.be — 2026

