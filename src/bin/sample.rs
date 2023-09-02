use std::{thread, time, env::consts};

fn main() {
    println!("Basic usage:");
    basic_usage(); 

    println!("Basic usage with helper:");
    basic_usage_with_helper(); 

    println!("Extended usage:");
    extended_usage();
}

fn basic_usage() {
    let duration = time::Duration::from_secs(1); 
    let start = tick_counter::start();
    thread::sleep(duration);
    let elapsed_ticks = tick_counter::stop() - start;
    println!("Number of elapsed ticks in {:?}: {}", duration, elapsed_ticks);
}

fn basic_usage_with_helper() {
    use tick_counter::TickCounter;

    let duration = time::Duration::from_secs(1); 
    let tick_counter = TickCounter::current();
    thread::sleep(duration);
    let elapsed_ticks = tick_counter.elapsed();
    println!("Number of elapsed ticks in {:?}: {}", duration, elapsed_ticks);
}

fn extended_usage() {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn main_test() {
        main();
    }

    #[test]
    fn basic_usage_test() {
        basic_usage();
    }

    #[test]
    fn basic_usage_with_helper_test() {
        basic_usage_with_helper();
    }

    #[test]
    fn extended_usage_test() {
        extended_usage();
    }
}
