# Hardware-based tick counters for high-precision benchmarks

[![crates.io](https://img.shields.io/crates/v/tick_counter)](https://crates.io/crates/tick_counter)
[![docs](https://img.shields.io/docsrs/tick_counter)](https://docs.rs/tick_counter/latest/tick_counter/)
[![build & test](https://github.com/sheroz/tick_counter/actions/workflows/ci.yml/badge.svg)](https://github.com/sheroz/tick_counter/actions/workflows/ci.yml)
[![license](https://img.shields.io/github/license/sheroz/tick_counter)](https://github.com/sheroz/tick_counter/blob/main/LICENSE.txt)

x86_64: executes [RDTSC](https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/ia-32-ia-64-benchmark-code-execution-paper.pdf) CPU instruction to read the time-stamp counter.

AArch64: reads value of the [CNTVCT_EL0](https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/CNTVCT-EL0--Counter-timer-Virtual-Count-register) counter-timer register.

## Tested on platforms

```text
x86_64 (Intel® Core™ i7)
AArch64 (Apple M1 Pro)
```

## Usage

For usage samples please look at [src/bin/sample.rs](src/bin/sample.rs)

### Basic usage

```rust
let start = tick_counter::start();
// ... lines of code to benchmark
let elapsed_ticks = tick_counter::stop() - start;
println!("Number of elapsed ticks: {}", elapsed_ticks);
```

### Basic usage with helper

```rust
use tick_counter::TickCounter;
 
let tick_counter = TickCounter::current();
// ... lines of code to benchmark
let elapsed_ticks = tick_counter.elapsed();
println!("Number of elapsed ticks: {}", elapsed_ticks);
```

### Extended usage

```rust
use std::{thread, time, env::consts};

println!("Environment: {}/{} {}", consts::OS, consts::FAMILY, consts::ARCH);

let (counter_frequency, accuracy) = tick_counter::frequency();
println!("Tick frequency, MHZ: {}", counter_frequency as f64 / 1e6_f64);
let estimation_source = match accuracy {
    tick_counter::TickCounterFrequencyBase::Hardware => "hardware".to_string(),
    tick_counter::TickCounterFrequencyBase::Measured(duration) => format!("software, estimated in {:?}", duration)
};
println!("Tick frequency is provided by: {}", estimation_source);

let counter_accuracy = tick_counter::precision(counter_frequency);
println!("Tick accuracy, nanoseconds: {}", counter_accuracy);

let counter_start = tick_counter::start();
thread::sleep(time::Duration::from_secs(1));
let counter_stop = tick_counter::stop();

println!("Tick counter start: {}", counter_start);
println!("Tick counter stop: {}", counter_stop);

let elapsed_ticks = counter_stop - counter_start;
println!("Elapsed ticks count in ~1 seconds thread::sleep(): {}", elapsed_ticks);

let elapsed_nanoseconds = (elapsed_ticks as f64) * counter_accuracy;
println!("Elapsed nanoseconds according to elapsed ticks: {}", elapsed_nanoseconds);
```

### Outputs

#### 1. Macbook Pro 16 2021 / Apple Silicon

```text
Apple M1 Pro
MacOS Ventura 13.5.1, Darwin Kernel Version 22.6.0
```

Output

```text
Basic usage:
Number of elapsed ticks in 1s: 24120997

Basic usage with helper:
Number of elapsed ticks in 1s: 24122097

Extended usage:
Environment: macos/unix aarch64
Tick frequency, MHZ: 24.00
Tick frequency is provided by: hardware
Tick accuracy, nanoseconds: 41.67
Tick counter start: 103684134140
Tick counter stop: 103708255194
Elapsed ticks count in ~1 seconds thread::sleep(): 24121054
Elapsed nanoseconds according to elapsed ticks: 1005043916.67

Comparing the measurement methods using 100 samples:
Elapsed time in nanoseconds, using std::time::Instant
  Mean = 60.34
  Min  = 41.00
  Max  = 167.00
  Standard deviation = 23.92 (39.64 %)
Elapsed time in nanoseconds, using tick_counter
  Mean = 42.41
  Min  = 42.00
  Max  = 83.00
  Standard deviation = 4.08 (9.62 %)
```

#### 2. Ubuntu 22.04 LTS / Intel® Core™ i7

```text
Intel(R) Core(TM) i7-3770 CPU @ 3.40GHz
Linux 6.2.0-31-generic #31~22.04.1-Ubuntu
```

Output

```text
Basic usage:
Number of elapsed ticks in 1s: 3430495113
---
Basic usage with helper:
Number of elapsed ticks in 1s: 3430495231
---
Extended usage:
Environment: linux/unix x86_64
Tick frequency, MHZ: 3430.494694
Tick frequency is provided by: software, estimated in 1s
Tick accuracy, nanoseconds: 0.29150314727173865
Tick counter start: 53632533092006
Tick counter stop: 53635963587302
Elapsed ticks count in ~1 seconds thread::sleep(): 3430495296
Elapsed nanoseconds according to elapsed ticks: 1000000175.4848946
---
Comparing the measurement methods using 100 samples:
Elapsed time in nanoseconds, using std::time::Instant
  Mean = 46.01
  Min  = 39.00
  Max  = 418.00
  Standard deviation = 37.46 (81.42 %)
-
Elapsed time in nanoseconds, using tick_counter
  Mean = 16.52
  Min  = 15.00
  Max  = 17.00
  Standard deviation = 0.85 (5.17 %)
---
```
