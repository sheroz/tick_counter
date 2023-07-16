# Hardware-based tick counters for high-precision benchmarks

<a href="https://crates.io/crates/tick_counter" target="_blank"><img src="https://img.shields.io/crates/v/tick_counter"/></a>
<a href="https://docs.rs/tick_counter/latest/tick_counter/" target="_blank"><img src="https://img.shields.io/docsrs/tick_counter"/></a>
<a href="https://github.com/sheroz/tick_counter/actions/workflows/ci.yml" target="_blank"><img src="https://github.com/sheroz/tick_counter/actions/workflows/ci.yml/badge.svg"/></a>
<a href="https://github.com/sheroz/tick_counter/blob/main/LICENSE.txt" target="_blank"><img src="https://img.shields.io/github/license/sheroz/tick_counter"/></a>

x86_64: executes [RDTSC](https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/ia-32-ia-64-benchmark-code-execution-paper.pdf) CPU instruction to read the time-stamp counter.

AArch64: reads value of the [CNTVCT_EL0](https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/CNTVCT-EL0--Counter-timer-Virtual-Count-register) counter-timer register.

## Tested on platforms

    x86_64 (Intel® Core™ i7)
    AArch64 (Apple M1 Pro)

## Usage

For usage samples please look at [src/bin/sample.rs](src/bin/sample.rs)

### Basic usage

    use tick_counter::*;
    let start = tick_counter_start();
    // ... lines of code to benchmark
    let elapsed_ticks = tick_counter_stop() - start;
    println!("Number of elapsed ticks: {}", elapsed_ticks);

### Sample usage

    use std::{thread, time, env::consts};
    use tick_counter::*;

    println!("\nEnvironment: {}/{} {}", consts::OS, consts::FAMILY, consts::ARCH);

    let (counter_frequency, accuracy) = tick_counter_frequency();
    println!("Tick frequency, MHZ: {}", counter_frequency as f64 / 1e6_f64);
    let estimation_source = match accuracy {
        TickCounterFrequencyBase::Hardware => "hardware".to_string(),
        TickCounterFrequencyBase::Measured(duration) => format!("software, estimated in {:?}", duration)
    };
    println!("Tick frequency is provided by: {}", estimation_source);

    let counter_accuracy = tick_counter_precision_nanoseconds(counter_frequency);
    println!("Tick accuracy, nanoseconds: {}", counter_accuracy);

    let counter_start = tick_counter_start();
    thread::sleep(time::Duration::from_secs(1));
    let counter_stop = tick_counter_stop();

    println!("Tick counter start: {}", counter_start);
    println!("Tick counter stop: {}", counter_stop);
    
    let elapsed_ticks = counter_stop - counter_start;
    println!("Elapsed ticks count in ~1 seconds thread::sleep(): {}", elapsed_ticks);

    let elapsed_nanoseconds = (elapsed_ticks as f64) * counter_accuracy;
    println!("Elapsed nanoseconds according to elapsed ticks: {}", elapsed_nanoseconds);

### Outputs

#### 1. Macbook Pro 16 2021 / Apple Silicon

    Apple M1 Pro
    MacOS Ventura 13.4, Darwin Kernel Version 22.5.0

    Output:

    Environment: macos/unix aarch64
    Tick frequency, MHZ: 24
    Tick frequency is provided by: hardware
    Tick accuracy, nanoseconds: 41.666666666666664
    Tick counter start: 48031196281005
    Tick counter stop: 48031220402058
    Elapsed ticks count in ~1 seconds thread::sleep(): 24121053
    Elapsed nanoseconds according to elapsed ticks: 1005043875

#### 2. Ubuntu 22.04 LTS / Intel® Core™ i7

    Intel(R) Core(TM) i7-3770 CPU @ 3.40GHz
    Linux 5.19.0-46-generic #47~22.04.1-Ubuntu

    Output:

    Environment: linux/unix x86_64
    Tick frequency, MHZ: 3430.481526
    Tick frequency is provided by: software, estimated in 1s
    Tick accuracy, nanoseconds: 0.29150426621478326
    Tick counter start: 9639567570396
    Tick counter stop: 9642998073707
    Elapsed ticks count in ~1 seconds thread::sleep(): 3430503311
    Elapsed nanoseconds according to elapsed ticks: 1000006350.4204394
