# CPU-based hardware tick counters for nanoseconds resolution benchmarks

## Tested on platforms

    AArch64 (Apple Silicon M1 Pro)
    x86_64 (Intel® Core™ i7)

## Sample usage

### Please look at src/bin/sample.rs

    let counter_frequency = estimated_counter_frequency();
    println!("Estimated counter frequency, MHZ: {}", counter_frequency as f64 / 1e6_f64);

    let counter_accuracy = expected_counter_accuracy_nanoseconds(counter_frequency);
    println!("Expected counter accuracy, Nanoseconds: {}", counter_accuracy);

    let counter_start = tick_counter_start();
    thread::sleep(time::Duration::from_secs(1));
    let counter_stop = tick_counter_stop();

    println!("Counter start: {}", counter_start);
    println!("Counter stop: {}", counter_stop);
    let elapsed_ticks = counter_stop - counter_start;
    println!("Elapsed ticks count in ~1 second: {}", elapsed_ticks);

    let elapsed_nanoseconds = (elapsed_ticks as f64) * counter_accuracy;
    println!("Elapsed nanoseconds according to elapsed ticks: {}", elapsed_nanoseconds);

### Sample output

#### Platform

    Linux 5.19.0-46-generic #47~22.04.1-Ubuntu
    Intel(R) Core(TM) i7-3770 CPU @ 3.40GHz

#### Output

    Environment: linux/unix x86_64
    Estimated counter frequency, MHZ: 3430.141314
    Expected counter accuracy, Nanoseconds: 0.29153317850740884
    Counter start: 14286143200274
    Counter stop: 14289573388952
    Elapsed ticks count in ~1 second: 3430188678
    Elapsed nanoseconds according to elapsed ticks: 1000013808.1774668
