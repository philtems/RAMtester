A lightweight, high-performance RAM testing tool written in Rust — designed to stress-test memory, detect hardware faults, and provide detailed error analysis.

RAMtester fills your system memory with pseudo-random patterns, verifies data integrity, and reports mismatches with address-level precision. Ideal for diagnosing faulty RAM, overclocking stability, or simply benchmarking your system.

✨ Features

    ✅ Memory allocation up to available RAM — specify size in MB/GB or use MAX

    ✅ Write-read-verify cycle
    
    ✅ Real-time progress display with time estimates and color-coded console output

    ✅ Detailed error reporting — exact address, expected vs actual value

    ✅ Visual error map — 2D grid showing faulty memory regions

    ✅ Loop mode (--loop) for continuous testing with random patterns

    ✅ Low overhead — compiled with release optimizations, panic=abort, stripped binary

    ✅ Automatic language detection — English / French (based on LANG env variable)

    ✅ No external dependencies (except rand for random pattern generation)

Installation

git clone https://github.com/philtems/RAMtester.git
cd ramtester
cargo build --release
./target/release/ramtester

🚀 Usage

ramtester <size> [--loop]

Argument	Description
<size>	Memory size to test. Examples: 512M, 2G, MAX
--loop	Run continuously with random patterns (press Ctrl+C to stop)

Examples

# Test 1 GB of RAM once
ramtester 1G

# Test all available memory (automatically adjusts if loop mode)
ramtester MAX

# Continuous testing with 512 MB, random patterns each loop
ramtester 512M --loop

📊 Output example

RAMtester v3.0
2025, Philippe TEMESI
https://www.tems.be

Total system memory: 32768.0 MB.
Available memory: 31876542464 bytes (30400 MB)

Filling memory...
[ 100.00%] Filling - Elapsed:     12s - Remaining:      0s
Filling completed in 12.34s

Verifying memory...
[ 100.00%] Verifying - Elapsed:     15s - Remaining:      0s
Verification completed in 15.67s

✓ Test successful: no errors detected.

===== Summary =====
Total errors detected: 0
Total memory tested: 1073741824 bytes (1024 MB)
Total time elapsed: 28.01s

If errors are found:

✗ Test failed: 42 errors.

Memory Error Map (42 total errors)
1 char = 7340032 bytes
+----------------------------------------------------------------------+
|..............#..............#.......................................|
|......................................#..............................|
...

Press ESC to continue...

⚙️ How it works

    Allocation — Requests a contiguous block of memory of the specified size.

    Filling — Writes a deterministic, address-dependent pattern to every byte:

value = (address XOR pattern) % 256

    Verification — Reads back each byte and compares with the expected value.

    Reporting — Any mismatch is logged with the exact memory address, expected and actual values.

    Error map — Visual representation of error density across the allocated block.

In --loop mode, each iteration uses a random 8-bit pattern, increasing coverage over time.
🛠️ Technical details
Memory safety

    Written in 100% safe Rust — no unsafe code.

    Memory is allocated via Vec<u8>, automatically freed when the test completes or the program exits.

Performance optimizations
Setting	Effect
opt-level = "z"	Optimize for binary size
lto = true	Link-time optimization
codegen-units = 1	Better optimizations at the cost of build time
panic = "abort"	No unwinding tables — smaller binary
strip = true	Removes debug symbols

Resulting binary is < 500 KB statically linked.
Platform support

Currently Linux only (reads /proc/meminfo for available memory).



# Continuous testing with 512 MB, random patterns each loop
ramtester 512M --loop
