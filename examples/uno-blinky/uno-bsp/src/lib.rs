#![cfg_attr(target_arch = "avr", no_std)]

pub mod led;
pub mod registers;
pub mod serial;
pub mod timer;

use led::PinLed;
use registers::AvrRegisters;
use serial::PollingSerial;
use timer::UnoTimer;

pub struct UnoBsp<R: AvrRegisters> {
    pub timer: UnoTimer<R>,
    pub led: PinLed<R>,
    pub serial: PollingSerial<R>,
}

impl<R: AvrRegisters + Clone> UnoBsp<R> {
    pub fn new(regs: R, ocr1a_value: u16) -> Self {
        let timer = UnoTimer::new(regs.clone(), ocr1a_value);
        let led = PinLed::new(regs.clone());
        let serial = PollingSerial::new(regs);

        Self { timer, led, serial }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use registers::MockRegisters;
    use timer::OCF1A;
    use led::{Led, PB5};
    use serial::{Serial, UDRE0, RXC0, TXEN0, RXEN0, UCSZ01, UCSZ00};
    use wrtos_core::Timer;

    #[test]
    fn test_registers_mock() {
        let mut regs = MockRegisters::new();
        regs.write_tccr1a(0x55);
        assert_eq!(regs.get_tccr1a(), 0x55);
        
        regs.write_tccr1b(0xAA);
        assert_eq!(regs.get_tccr1b(), 0xAA);

        regs.write_ocr1a(12345);
        assert_eq!(regs.get_ocr1a(), 12345);

        regs.write_tcnt1(500);
        assert_eq!(regs.get_tcnt1(), 500);

        regs.set_tifr1(0x02);
        assert_eq!(regs.read_tifr1(), 0x02);
        // Writing 1 to bit 1 should clear it
        regs.write_tifr1(0x02);
        assert_eq!(regs.read_tifr1(), 0);

        regs.write_ddrb(0x0F);
        assert_eq!(regs.read_ddrb(), 0x0F);

        regs.write_portb(0x10);
        assert_eq!(regs.read_portb(), 0x10);

        // PINB toggle
        regs.write_pinb(0x10);
        assert_eq!(regs.read_portb(), 0x00);
        assert_eq!(regs.read_pinb(), 0x10);

        regs.write_ubrr0(103);
        assert_eq!(regs.get_ubrr0(), 103);

        regs.write_ucsr0b(0x08);
        assert_eq!(regs.get_ucsr0b(), 0x08);

        regs.write_ucsr0c(0x06);
        assert_eq!(regs.get_ucsr0c(), 0x06);

        regs.set_ucsr0a(0x20);
        assert_eq!(regs.read_ucsr0a(), 0x20);

        regs.write_udr0(0x41);
        assert_eq!(regs.read_udr0(), 0x41);
        assert_eq!(regs.get_udr0(), 0x41);
    }

    #[test]
    fn test_timer() {
        let regs = MockRegisters::new();
        let mut timer = timer::UnoTimer::new(regs.clone(), 24999);

        // Verify initialization
        assert_eq!(regs.get_tccr1a(), 0);
        assert_eq!(regs.get_tcnt1(), 0);
        assert_eq!(regs.get_ocr1a(), 24999);
        assert_eq!(regs.get_tccr1b(), (1 << timer::WGM12) | (1 << timer::CS11) | (1 << timer::CS10));

        // Test wait_next_tick normal path
        regs.set_tifr1(1 << OCF1A);
        assert!(!timer.has_overrun());
        timer.wait_next_tick();
        assert_eq!(regs.read_tifr1() & (1 << OCF1A), 0);
        assert!(!timer.has_overrun());

        // Test wait_next_tick overrun path
        regs.set_sticky_tifr1(true);
        regs.set_tifr1(1 << OCF1A);
        timer.wait_next_tick();
        assert!(timer.has_overrun());

        // Test reset_overrun
        timer.reset_overrun();
        assert!(!timer.has_overrun());

        // Test get_regs_mut and release
        let mut timer2 = timer::UnoTimer::new(regs.clone(), 1000);
        timer2.get_regs_mut().write_ocr1a(2000);
        let released = timer2.release();
        assert_eq!(released.get_ocr1a(), 2000);
    }

    #[test]
    fn test_led() {
        let regs = MockRegisters::new();
        let mut led = PinLed::new(regs.clone());

        assert_eq!(regs.get_ddrb(), 0);
        led.init();
        assert_eq!(regs.get_ddrb(), 1 << PB5);

        assert!(!led.is_on());
        led.set_high();
        assert!(led.is_on());
        assert_eq!(regs.get_portb(), 1 << PB5);

        led.set_low();
        assert!(!led.is_on());
        assert_eq!(regs.get_portb(), 0);

        led.toggle();
        assert!(led.is_on());
        assert_eq!(regs.get_portb(), 1 << PB5);

        led.toggle();
        assert!(!led.is_on());
        assert_eq!(regs.get_portb(), 0);

        // Test get_regs_mut and release
        let mut led2 = PinLed::new(regs.clone());
        led2.get_regs_mut().write_portb(0xFF);
        let released = led2.release();
        assert_eq!(released.get_portb(), 0xFF);
    }

    #[test]
    fn test_serial() {
        let regs = MockRegisters::new();
        let mut serial = PollingSerial::new(regs.clone());

        serial.init(103);
        assert_eq!(regs.get_ubrr0(), 103);
        assert_eq!(regs.get_ucsr0b(), (1 << TXEN0) | (1 << RXEN0));
        assert_eq!(regs.get_ucsr0c(), (1 << UCSZ01) | (1 << UCSZ00));

        // Test write_byte
        regs.set_ucsr0a(1 << UDRE0);
        serial.write_byte(0x41);
        assert_eq!(regs.get_udr0(), 0x41);

        // Test read_byte (nothing received)
        regs.set_ucsr0a(0);
        assert_eq!(serial.read_byte(), None);

        // Test read_byte (byte received)
        regs.set_ucsr0a(1 << RXC0);
        regs.set_udr0(0x42);
        assert_eq!(serial.read_byte(), Some(0x42));

        // Test get_regs_mut and release
        let mut serial2 = PollingSerial::new(regs.clone());
        serial2.get_regs_mut().write_udr0(0xAA);
        let released = serial2.release();
        assert_eq!(released.get_udr0(), 0xAA);
    }

    #[test]
    fn test_bsp_creation() {
        let regs = MockRegisters::new();
        let _bsp = UnoBsp::new(regs, 24999);
    }
}
