# CPU-based hardware tick counters for nanoseconds resolution benchmarks

## Tested on platforms

    AArch64 (Apple Silicon M1 Pro)
    x86_64 (Intel® Core™ i7)

## Sample usage

### Please look at src/bin/sample.rs

    println!("\nEnvironment: {}/{} {}", consts::OS, consts::FAMILY, consts::ARCH);

    let (counter_frequency, accuracy) = tick_counter_frequency();
    println!("Tick frequency, MHZ: {}", counter_frequency as f64 / 1e6_f64);
    let estimation_source = match accuracy {EstimationAccuracy::Hardware => "hardware", _ => "software estimated"};
    println!("Tick frequency provided by: {}", estimation_source);

    let counter_accuracy = tick_counter_accuracy_nanoseconds(counter_frequency);
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

### Sample outputs

#### 1. Macbook Pro 16 2021

    Apple M1 Pro
    MacOS Ventura 13.4, Darwin Kernel Version 22.5.0

    Output:

    Environment: macos/unix aarch64
    Tick frequency, MHZ: 24
    Tick frequency provided by: hardware
    Tick accuracy, nanoseconds: 41.666666666666664
    Tick counter start: 48031196281005
    Tick counter stop: 48031220402058
    Elapsed ticks count in ~1 seconds thread::sleep(): 24121053
    Elapsed nanoseconds according to elapsed ticks: 1005043875

#### 2. Ubuntu 22.04 LTS

    Intel(R) Core(TM) i7-3770 CPU @ 3.40GHz
    Linux 5.19.0-46-generic #47~22.04.1-Ubuntu

    Output:

    Environment: linux/unix x86_64
    Tick frequency, MHZ: 3430.499961
    Tick frequency provided by: software estimated
    Tick accuracy, nanoseconds: 0.2915026997139208
    Tick counter start: 42091578596094
    Tick counter stop: 42095009119616
    Elapsed ticks count in ~1 seconds thread::sleep(): 3430523522
    Elapsed nanoseconds according to elapsed ticks: 1000006868.0951079
