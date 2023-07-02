use std::{thread, time};
use std::env::consts;
use cpu_tick_counter::*;

fn main() {
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
}