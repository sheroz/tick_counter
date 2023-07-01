use std::{thread, time};
use std::env::consts;
use cpu_counter::*;

fn main() {
    println!("\nEnvironment: {}/{} {}", consts::OS, consts::FAMILY, consts::ARCH);

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
}