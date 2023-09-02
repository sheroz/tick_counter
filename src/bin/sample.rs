use std::{thread, time::{self, Instant}, env::consts};

fn main() {
    basic_usage(); 
    basic_usage_with_helper(); 
    extended_usage();
    compare_with_time_instant();
}

fn basic_usage() {
    println!("Basic usage:");
    let duration = time::Duration::from_secs(1); 
    let start = tick_counter::start();
    thread::sleep(duration);
    let elapsed_ticks = tick_counter::stop() - start;
    println!("Number of elapsed ticks in {:?}: {}", duration, elapsed_ticks);

    println!("---");
}

fn basic_usage_with_helper() {
    use tick_counter::TickCounter;

    println!("Basic usage with helper:");
    let duration = time::Duration::from_secs(1); 
    let tick_counter = TickCounter::current();
    thread::sleep(duration);
    let elapsed_ticks = tick_counter.elapsed();
    println!("Number of elapsed ticks in {:?}: {}", duration, elapsed_ticks);

    println!("---");
}

fn extended_usage() {
    println!("Extended usage:");
    
    println!("Environment: {}/{} {}", consts::OS, consts::FAMILY, consts::ARCH);

    let (counter_frequency, accuracy) = tick_counter::frequency();
    println!("Tick frequency, MHZ: {}", counter_frequency as f64 / 1e6_f64);
    let estimation_source = match accuracy {
        tick_counter::TickCounterFrequencyBase::Hardware => "hardware".to_string(),
        tick_counter::TickCounterFrequencyBase::Measured(duration) => format!("software, estimated in {:?}", duration)
    };
    println!("Tick frequency is provided by: {}", estimation_source);

    let counter_accuracy = tick_counter::precision_nanoseconds(counter_frequency);
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

    println!("---");
}

fn compare_with_time_instant() {
    println!("Comparing the measurement results:");

    println!("Measurement results using 'std::time::Instant'");
    for _ in 0..10 {
        let time = Instant::now();
        let elapsed_time = time.elapsed();
        println!("Elapsed time {:?}", elapsed_time);
    }

    println!("-");

    println!("Measurement results using 'tick_counter'");
    let (counter_frequency,_) = tick_counter::frequency();
    let counter_precision = tick_counter::precision_nanoseconds(counter_frequency);
    for _ in 0..10 {
        let counter_start = tick_counter::start();
        let elapsed_ticks = tick_counter::stop() - counter_start;
        let elapsed_time = counter_precision * elapsed_ticks as f64;
        println!("Elapsed ticks: {}, elapsed time, {:.2} ns", elapsed_ticks, elapsed_time);
    }

    println!("---");
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

    #[test]
    fn compare_with_time_instant_test() {
        compare_with_time_instant();
    }
}
