use std::{time::Duration, arch::asm};

pub enum TickCounterFrequencyBase {
    Hardware,
    Measured(Duration)
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub fn x86_64_tick_counter() -> u64 {
    let mut reg_eax: u32;
    let mut reg_edx: u32;

    unsafe {
        asm!("rdtsc", out("eax") reg_eax, out("edx") reg_edx);
    }

    (reg_edx as u64) << 32 | reg_eax as u64
}

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

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn tick_counter_start() -> u64 {
    aarch64_tick_counter()
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn tick_counter_stop() -> u64 {
    aarch64_tick_counter()
}

#[cfg(target_arch = "aarch64")]
#[inline]
pub fn tick_counter_frequency() -> (u64, TickCounterFrequencyBase) {
    let counter_frequency: u64;
    unsafe {
        asm!(
            "mrs x0, cntfrq_el0",
            out("x0") counter_frequency
        );
    }
    (counter_frequency, TickCounterFrequencyBase::Hardware)
}

#[cfg(target_arch = "x86_64")]
pub fn tick_counter_frequency() -> (u64, TickCounterFrequencyBase)  {
    let measure_duration = Duration::from_millis(1000);
    let frequency_base = TickCounterFrequencyBase::Measured(measure_duration);
    (measure_tick_counter_frequency(&frequency_base), frequency_base)
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub fn x86_64_tick_counter_processor_id() -> (u64, u32) {
    let mut reg_eax: u32;
    let mut reg_edx: u32;
    let mut reg_ecx: u32;

    unsafe {
        asm!("rdtscp", out("eax") reg_eax, out("edx") reg_edx, out("ecx") reg_ecx);
    }

    ((reg_edx as u64) << 32 | reg_eax as u64, reg_ecx)
}

#[cfg(target_arch = "x86_64")]
#[inline]
pub fn tick_counter_start() -> u64 {
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

#[cfg(target_arch = "x86_64")]
#[inline]
pub fn tick_counter_stop() -> u64 {
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

#[cfg(target_arch = "x86_64")]
pub fn measure_tick_counter_frequency(accuracy: &TickCounterFrequencyBase) -> u64 {
    use std::thread;

    let measure_duration = match accuracy {
        TickCounterFrequencyBase::Measured(duration) => duration,
        TickCounterFrequencyBase::Hardware => panic!("Hardware frequency is not provided in x86_64 platfrom.")
    };

    let counter_start = tick_counter_start();
    thread::sleep(*measure_duration);
    let counter_stop = tick_counter_stop();
    (((counter_stop - counter_start) as f64) / measure_duration.as_secs_f64()) as u64
}

pub fn tick_counter_precision_nanoseconds(frequency: u64) -> f64{
    1.0e9_f64 / (frequency as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64_tick_counter() {
        use std::{thread, time};

        let counter_start = tick_counter_start();
        thread::sleep(time::Duration::from_millis(1));
        let counter_stop = tick_counter_stop();
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

        let counter_start = tick_counter_start();
        let counter_stop = tick_counter_stop();
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

        let (counter7, cpu_core_id_3) = x86_64_tick_counter_processor_id();
        let (counter8, cpu_core_id_4) = x86_64_tick_counter_processor_id();
        let diff_tick_asm_rdtscp = counter8 - counter7;

        assert!(counter7 < counter8);
        assert!(cpu_core_id_1 == cpu_core_id_3);
        assert!(cpu_core_id_3 == cpu_core_id_4);
        assert!(diff_tick_asm_rdtscp > 0);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn test_x86_64_counter_frequency() {
        let (counter_frequency, frequency_base) = tick_counter_frequency();
        assert!(counter_frequency > 0);
        let estimated_duration = match frequency_base {
            TickCounterFrequencyBase::Hardware => None,
            TickCounterFrequencyBase::Measured(duration) => Some(duration)
        };
        assert_eq!(estimated_duration, Some(Duration::from_millis(1000)));
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    #[should_panic(expected = "Hardware frequency is not provided in x86_64 platfrom.")]
    fn test_x86_64_measure_tick_counter_frequency() {
        measure_tick_counter_frequency(&TickCounterFrequencyBase::Hardware);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64_counter_frequency() {
        let (counter_frequency, frequency_base) = tick_counter_frequency();
        assert!(counter_frequency > 0);
        assert_eq!(frequency_base, TickCounterFrequencyBase::Hardware);
    }

    #[test]
    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    fn test_counter_accuracy() {
        let counter_frequency = 24_000_000;
        let counter_accuracy = tick_counter_precision_nanoseconds(counter_frequency);
        assert_eq!((counter_accuracy as u64), 41);
    }
}
