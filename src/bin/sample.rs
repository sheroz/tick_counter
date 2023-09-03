use std::thread;
use std::time::{self, Instant};
use std::env::consts;

fn main() {
    basic_usage(); 
    println!();
    basic_usage_with_helper(); 
    println!();
    extended_usage();
    println!();
    compare_with_time_instant();
}

fn basic_usage() {
    println!("Basic usage:");
    let duration = time::Duration::from_secs(1); 
    let start = tick_counter::start();
    thread::sleep(duration);
    let elapsed_ticks = tick_counter::stop() - start;
    println!("Number of elapsed ticks in {:?}: {}", duration, elapsed_ticks);
}

fn basic_usage_with_helper() {
    use tick_counter::TickCounter;

    println!("Basic usage with helper:");
    let duration = time::Duration::from_secs(1); 
    let tick_counter = TickCounter::current();
    thread::sleep(duration);
    let elapsed_ticks = tick_counter.elapsed();
    println!("Number of elapsed ticks in {:?}: {}", duration, elapsed_ticks);
}

fn extended_usage() {
    println!("Extended usage:");
    println!("Environment: {}/{} {}", consts::OS, consts::FAMILY, consts::ARCH);

    let (counter_frequency, accuracy) = tick_counter::frequency();
    let frequency_base = match accuracy {
        tick_counter::TickCounterFrequencyBase::Hardware => "hardware provided".to_string(),
        tick_counter::TickCounterFrequencyBase::Measured(duration) => format!("software estimated in {:?}", duration)
    };
    println!("Tick frequency, MHZ: {:.2} ({})", counter_frequency as f64 / 1e6_f64, frequency_base);

    let counter_accuracy = tick_counter::precision_nanoseconds(counter_frequency);
    println!("Tick accuracy, nanoseconds: {:.2}", counter_accuracy);

    let counter_start = tick_counter::start();
    thread::sleep(time::Duration::from_secs(1));
    let counter_stop = tick_counter::stop();

    println!("Tick counter start: {}", counter_start);
    println!("Tick counter stop: {}", counter_stop);
    
    let elapsed_ticks = counter_stop - counter_start;
    println!("Elapsed ticks count in 1 seconds: {}", elapsed_ticks);

    let elapsed_nanoseconds = (elapsed_ticks as f64) * counter_accuracy;
    println!("Elapsed nanoseconds according to elapsed ticks: {:.2}", elapsed_nanoseconds);
}

fn calculate_statistics (samples: &[f64]) {
    let mean = samples.iter().sum::<f64>() / (samples.len() as f64);
    let min = samples.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = samples.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    println!("  Mean = {:.2}", mean);
    println!("  Min  = {:.2}", min);
    println!("  Max  = {:.2}", max);

    let deviation = f64::sqrt(samples.iter().map(|v| {
        let diff = mean - *v;
        diff * diff
    }).sum::<f64>() / samples.len() as f64);
    println!("  Standard deviation = {:.2} ({:.2} %)", deviation, 100.0 * deviation / mean);
}

fn compare_with_time_instant() {
    const SAMPLES_COUNT: usize = 100;

    println!("Comparing the measurement methods using {} samples:", SAMPLES_COUNT);

    let mut samples = Vec::<f64>::with_capacity(SAMPLES_COUNT);

    println!("Elapsed time in nanoseconds, using std::time::Instant");
    for _ in 0..SAMPLES_COUNT {
        let time = Instant::now();
        let elapsed_time = time.elapsed();
        samples.push(elapsed_time.as_nanos() as f64);
    }
    calculate_statistics(&mut samples);

    samples.clear();
    println!("Elapsed time in nanoseconds, using tick_counter");
    let (counter_frequency,_) = tick_counter::frequency();
    let counter_precision = tick_counter::precision_nanoseconds(counter_frequency);
    for _ in 0..SAMPLES_COUNT {
        let counter_start = tick_counter::start();
        let elapsed_ticks = tick_counter::stop() - counter_start + 1;
        let elapsed_time = counter_precision * elapsed_ticks as f64;
        samples.push(elapsed_time.round());
    }
    calculate_statistics(&mut samples);
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
