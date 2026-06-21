#![cfg_attr(target_arch = "avr", no_std)]
#![cfg_attr(target_arch = "avr", no_main)]

#[cfg(not(target_arch = "avr"))]
use std::println;

use uno_bsp::led::Led;

// Include compile-time generated schedule
include!(concat!(env!("OUT_DIR"), "/schedule.rs"));

// Define global LED variable
#[cfg(target_arch = "avr")]
static mut GLOBAL_LED: Option<uno_bsp::led::PinLed<uno_bsp::registers::RealRegisters>> = None;

#[cfg(not(target_arch = "avr"))]
static mut GLOBAL_LED: Option<uno_bsp::led::PinLed<uno_bsp::registers::MockRegisters>> = None;

/// Task: Toggle the board's LED.
/// This matches the task name 'blink_led' defined in schedule.toml.
#[no_mangle]
pub fn blink_led() {
    unsafe {
        if let Some(ref mut led) = GLOBAL_LED {
            led.toggle();
        }
    }
}

// --- AVR Target Entry Point ---
#[cfg(target_arch = "avr")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    let regs = uno_bsp::registers::RealRegisters;

    // Initialize LED pin
    let mut led = uno_bsp::led::PinLed::new(regs);
    led.init();
    unsafe {
        GLOBAL_LED = Some(led);
    }

    // Setup Timer1: CTC mode, 16MHz clock, prescaler 64
    let ocr1a = (250000 / TICK_RATE_HZ) - 1;
    let timer = uno_bsp::timer::UnoTimer::new(regs, ocr1a as u16);

    // Start Cyclic Executive
    let mut exec = wrtos_core::CyclicExecutive::new(SCHEDULE);
    exec.run_loop(timer);
}

#[cfg(target_arch = "avr")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// --- Host Simulation Entry Point ---
#[cfg(not(target_arch = "avr"))]
fn main() {
    println!("=== Elegoo Arduino Uno R3 Blinky RTOS Simulation (Host Mode) ===");
    println!("Running the cyclic executive on the host using MockRegisters...");
    println!("Tick Rate: {} Hz ({} ms frames)", TICK_RATE_HZ, 1000 / TICK_RATE_HZ);
    println!("Major Cycle: {} frames ({} ms)\n", MAJOR_CYCLE_FRAMES, (1000 / TICK_RATE_HZ) * MAJOR_CYCLE_FRAMES as u32);

    let regs = uno_bsp::registers::MockRegisters::new();

    // Initialize LED pin
    let mut led = uno_bsp::led::PinLed::new(regs.clone());
    led.init();
    unsafe {
        GLOBAL_LED = Some(led);
    }

    // Setup Timer
    let ocr1a = (250000 / TICK_RATE_HZ) - 1;
    let mut timer = uno_bsp::timer::UnoTimer::new(regs.clone(), ocr1a as u16);
    let mut exec = wrtos_core::CyclicExecutive::new(SCHEDULE);

    println!("Starting simulation for 2 major cycles (20 frames)...");
    let mut last_led_state = false;

    for _ in 0..20 {
        // Trigger simulated timer tick compare match
        regs.set_tifr1(1 << uno_bsp::timer::OCF1A);

        let frame = exec.current_frame();
        
        if let Err(_) = exec.step(&mut timer) {
            println!("CRITICAL ERROR: Frame overrun in frame {}!", frame);
            std::process::exit(1);
        }

        let current_led_state = regs.get_portb() & (1 << uno_bsp::led::PB5) != 0;
        if current_led_state != last_led_state {
            last_led_state = current_led_state;
            println!(
                "Frame {:02}: LED state toggled -> {}",
                frame,
                if current_led_state { "ON  [🟢]" } else { "OFF [🔴]" }
            );
        } else {
            println!("Frame {:02}: (running, LED remains {})", frame, if current_led_state { "ON" } else { "OFF" });
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    println!("\nSimulation completed successfully.");
}

// --- Application Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use uno_bsp::registers::MockRegisters;

    #[test]
    fn test_blink_led_task() {
        let regs = MockRegisters::new();
        let mut led = uno_bsp::led::PinLed::new(regs.clone());
        led.init();
        unsafe {
            GLOBAL_LED = Some(led);
        }

        // Verify status using mock registers to avoid borrow-after-move
        assert_eq!(regs.get_portb() & (1 << uno_bsp::led::PB5), 0);
        blink_led();
        assert_ne!(regs.get_portb() & (1 << uno_bsp::led::PB5), 0);
        blink_led();
        assert_eq!(regs.get_portb() & (1 << uno_bsp::led::PB5), 0);
    }

    #[test]
    fn test_cyclic_executive_schedule() {
        let regs = MockRegisters::new();
        let mut led = uno_bsp::led::PinLed::new(regs.clone());
        led.init();
        unsafe {
            GLOBAL_LED = Some(led);
        }

        let ocr1a = (250000 / TICK_RATE_HZ) - 1;
        let mut timer = uno_bsp::timer::UnoTimer::new(regs.clone(), ocr1a as u16);
        let mut exec = wrtos_core::CyclicExecutive::new(SCHEDULE);

        // Frame 0: blink_led runs, LED turns ON
        regs.set_tifr1(1 << uno_bsp::timer::OCF1A);
        assert_eq!(exec.current_frame(), 0);
        let res = exec.step(&mut timer);
        assert_eq!(res, Ok(()));
        assert_ne!(regs.get_portb() & (1 << uno_bsp::led::PB5), 0);

        // Frames 1-4: no tasks, LED remains ON
        for expected_frame in 1..5 {
            assert_eq!(exec.current_frame(), expected_frame);
            regs.set_tifr1(1 << uno_bsp::timer::OCF1A);
            let res = exec.step(&mut timer);
            assert_eq!(res, Ok(()));
            assert_ne!(regs.get_portb() & (1 << uno_bsp::led::PB5), 0);
        }

        // Frame 5: blink_led runs, LED turns OFF
        assert_eq!(exec.current_frame(), 5);
        regs.set_tifr1(1 << uno_bsp::timer::OCF1A);
        let res = exec.step(&mut timer);
        assert_eq!(res, Ok(()));
        assert_eq!(regs.get_portb() & (1 << uno_bsp::led::PB5), 0);
    }
}
