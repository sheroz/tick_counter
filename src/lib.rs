//! # Hardware-based tick counters for high-precision benchmarks
//! * `x86_64`  - executes [RDTSC](https://www.intel.com/content/dam/www/public/us/en/documents/white-papers/ia-32-ia-64-benchmark-code-execution-paper.pdf) CPU instruction to read the time-stamp counter.
//! * `AArch64` - reads value of the [CNTVCT_EL0](https://developer.arm.com/documentation/ddi0595/2021-12/AArch64-Registers/CNTVCT-EL0--Counter-timer-Virtual-Count-register) counter-timer register.
//! 
//! ## Basic usage
//! 
//!```
//! let start = tick_counter::start();
//! // ... lines of code to benchmark
//! let elapsed_ticks = tick_counter::stop() - start;
//! println!("Number of elapsed ticks: {}", elapsed_ticks);
//!```

use std::{time::Duration, arch::asm};

/// The origin of the provided counter frequency
pub enum TickCounterFrequencyBase {
    /// Frequency is provided by hardware
    Hardware,

    /// Frequency is measured by counting number of ticks in `Duration` of time
    Measured(Duration)
}

/// Returns a current value of tick counter on `aarch64` architecture
#[cfg(target_arch = "aarch64")]
#[inline]
pub fn aarch64_tick_counter() -> u64 {
    let tick_counter: u64;
    unsafe {
        asm!(
            "mrs x0, cntvct_el0",
            out("x0") tick_counter
        );
    }
    tick_counter
}

/// Returns a current value of the tick counter to use as a staring point
#[cfg(target_arch = "aarch64")]
#[inline]
pub fn start() -> u64 {
    aarch64_tick_counter()
}

/// Returns a current value of the tick counter to use as a stopping point
#[cfg(target_arch = "aarch64")]
#[inline]
pub fn stop() -> u64 {
    aarch64_tick_counter()
}

/// Returns a frequency of tick counter in hertz (Hz)
/// * Returns a hardware-provided value of tick counter frequency on `aarch64` architecture.
/// * Returns a software-measured value of tick counter frequency on `x86_64` architecture measured in 1 second.
#[cfg(target_arch = "aarch64")]
#[inline]
pub fn frequency() -> (u64, TickCounterFrequencyBase) {
    let counter_frequency: u64;
    unsafe {
        asm!(
            "mrs x0, cntfrq_el0",
            out("x0") counter_frequency
        );
    }
    (counter_frequency, TickCounterFrequencyBase::Hardware)
}

/// Returns a frequency of tick counter in hertz (Hz)
/// * Returns a hardware-provided value of tick counter frequency on `aarch64` architecture.
/// * Returns a software-measured value of tick counter frequency on `x86_64` architecture measured in 1 second.
#[cfg(target_arch = "x86_64")]
pub fn frequency() -> (u64, TickCounterFrequencyBase)  {
    let measure_duration = Duration::from_secs(1);
    let frequency_base = TickCounterFrequencyBase::Measured(measure_duration);
    (x86_64_measure_frequency(&measure_duration), frequency_base)
}

/// Returns a current value of the tick counter based on Intel CPU's `RDTSC` instruction
/// 
/// This function is an aternative to Rust's core functions:
/// * `core::arch::x86::_rdtsc()`
/// * `core::arch::x86_64::_rdtsc()`
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub fn x86_64_tick_counter() -> u64 {
    let mut reg_eax: u32;
    let mut reg_edx: u32;

    unsafe {
        asm!("rdtsc", out("eax") reg_eax, out("edx") reg_edx);
    }

    (reg_edx as u64) << 32 | reg_eax as u64
}

/// Returns a tick counter and CPUID values based on Intel CPU's `RDTSCP` instruction
/// 
/// This function is an aternative to Rust's core functions:
/// * `core::arch::x86::_rdtscp()`
/// * `core::arch::x86_64::_rdtscp()`
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub fn x86_64_processor_id() -> (u64, u32) {
    let mut reg_eax: u32;
    let mut reg_edx: u32;
    let mut reg_ecx: u32;

    unsafe {
        asm!("rdtscp", out("eax") reg_eax, out("edx") reg_edx, out("ecx") reg_ecx);
    }

    ((reg_edx as u64) << 32 | reg_eax as u64, reg_ecx)
}

/// Returns a current value of the tick counter to use as a staring point
#[cfg(target_arch = "x86_64")]
#[inline]
pub fn start() -> u64 {
    let rax: u64;
    unsafe {
        asm!(
            "mfence",
            "lfence",
            "rdtsc",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") rax
        );
    }
    rax
}

/// Returns a current value of the tick counter to use as a stopping point
#[cfg(target_arch = "x86_64")]
#[inline]
pub fn stop() -> u64 {
    let rax: u64;
    unsafe {
        asm!(
            "rdtsc",
            "lfence",
            "shl rdx, 32",
            "or rax, rdx",
            out("rax") rax
        );
    }
    rax
}

/// Returns a measured value of tick counter frequency on `x86_64` architecture in hertz (Hz)
/// 
/// # Arguments
///
/// * `measure_duration` - A reference to `Duration` value 
#[cfg(target_arch = "x86_64")]
pub fn x86_64_measure_frequency(measure_duration: &Duration) -> u64 {
    use std::thread;
    let counter_start = start();
    thread::sleep(*measure_duration);
    let counter_stop = stop();
    (((counter_stop - counter_start) as f64) / measure_duration.as_secs_f64()) as u64
}

/// Returns a precision of tick counters in nanoseconds
pub fn precision(frequency: u64) -> f64{
    1.0e9_f64 / (frequency as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        use std::{thread, time};
        let start = start();
        thread::sleep(time::Duration::from_millis(20));
        let elapsed_ticks = stop() - start;
        assert!(elapsed_ticks > 0);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64_tick_counter() {
        use std::{thread, time};

        let counter_start = start();
        thread::sleep(time::Duration::from_millis(1));
        let counter_stop = stop();
        println!(
            "counter_start: {}. counter_end: {}",
            counter_start, counter_stop
        );
        assert!(counter_start < counter_stop);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_x86_64_counters() {
        use core::arch::x86_64::__rdtscp;
        use core::arch::x86_64::_rdtsc;

        let counter1 = x86_64_tick_counter();
        let counter2 = x86_64_tick_counter();
        let diff_tick_counter = counter2 - counter1;
        assert!(counter1 < counter2);
        assert!(diff_tick_counter > 0);

        let counter_start = start();
        let counter_stop = stop();
        let diff_tick_counter2 = counter_stop - counter_start;
        assert!(counter_start < counter_stop);
        assert!(diff_tick_counter2 > 0);

        let counter3 = unsafe { _rdtsc() };
        let counter4 = unsafe { _rdtsc() };
        let diff_tick_rdtsc = counter4 - counter3;
        assert!(counter3 < counter4);
        assert!(diff_tick_rdtsc > 0);

        let mut ecx: u32 = 0;
        let ptr_ecx: *mut u32 = (&mut ecx) as *mut u32;
        let counter5 = unsafe { __rdtscp(ptr_ecx) };
        let cpu_core_id_1 = ecx;

        let counter6 = unsafe { __rdtscp(ptr_ecx) };
        let cpu_core_id_2 = ecx;
        let diff_tick_rdtscp = counter6 - counter5;

        assert!(counter5 < counter6);
        assert!(diff_tick_rdtscp > 0);
        assert!(cpu_core_id_1 == cpu_core_id_2);

        let (counter7, cpu_core_id_3) = x86_64_processor_id();
        let (counter8, cpu_core_id_4) = x86_64_processor_id();
        let diff_tick_asm_rdtscp = counter8 - counter7;

        assert!(counter7 < counter8);
        assert!(cpu_core_id_1 == cpu_core_id_3);
        assert!(cpu_core_id_3 == cpu_core_id_4);
        assert!(diff_tick_asm_rdtscp > 0);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_x86_64_counter_frequency() {
        let (counter_frequency, frequency_base) = frequency();
        assert!(counter_frequency > 0);
        let estimated_duration = match frequency_base {
            TickCounterFrequencyBase::Hardware => None,
            TickCounterFrequencyBase::Measured(duration) => Some(duration)
        };
        assert_eq!(estimated_duration, Some(Duration::from_millis(1000)));
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64_counter_frequency() {
        let (counter_frequency, frequency_base) = frequency();
        assert!(counter_frequency > 0);
        match frequency_base   {
            TickCounterFrequencyBase::Hardware => (),
            _ => panic!("Unexpected frequency base!")
        }
    }

    #[test]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    fn test_counter_accuracy() {
        let counter_frequency = 24_000_000;
        let counter_accuracy = precision(counter_frequency);
        assert_eq!((counter_accuracy as u64), 41);
    }
}
