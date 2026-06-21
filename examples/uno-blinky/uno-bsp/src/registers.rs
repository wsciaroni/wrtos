#![allow(dead_code)]

pub trait AvrRegisters {
    // Timer 1 Control & Status Registers
    fn write_tccr1a(&mut self, val: u8);
    fn write_tccr1b(&mut self, val: u8);
    fn write_ocr1a(&mut self, val: u16);
    fn write_tcnt1(&mut self, val: u16);
    fn read_tifr1(&self) -> u8;
    fn write_tifr1(&mut self, val: u8);

    // GPIO (Port B) Registers
    fn write_ddrb(&mut self, val: u8);
    fn read_ddrb(&self) -> u8;
    fn write_portb(&mut self, val: u8);
    fn read_portb(&self) -> u8;
    fn write_pinb(&mut self, val: u8);
    fn read_pinb(&self) -> u8;

    // USART Registers (Polling Serial Driver)
    fn write_ubrr0(&mut self, val: u16);
    fn write_ucsr0b(&mut self, val: u8);
    fn write_ucsr0c(&mut self, val: u8);
    fn read_ucsr0a(&self) -> u8;
    fn write_udr0(&mut self, val: u8);
    fn read_udr0(&self) -> u8;
}

// Memory-mapped data addresses on ATmega328P
const TCCR1A_ADDR: *mut u8 = 0x80 as *mut u8;
const TCCR1B_ADDR: *mut u8 = 0x81 as *mut u8;
const TCNT1_ADDR: *mut u16 = 0x84 as *mut u16;
const OCR1A_ADDR: *mut u16 = 0x88 as *mut u16;
const TIFR1_ADDR: *mut u8 = 0x36 as *mut u8;

const PINB_ADDR: *mut u8 = 0x23 as *mut u8;
const DDRB_ADDR: *mut u8 = 0x24 as *mut u8;
const PORTB_ADDR: *mut u8 = 0x25 as *mut u8;

const UCSR0A_ADDR: *mut u8 = 0xC0 as *mut u8;
const UCSR0B_ADDR: *mut u8 = 0xC1 as *mut u8;
const UCSR0C_ADDR: *mut u8 = 0xC2 as *mut u8;
const UBRR0_ADDR: *mut u16 = 0xC4 as *mut u16;
const UDR0_ADDR: *mut u8 = 0xC6 as *mut u8;

/// The real register accessor using volatile read/write pointers.
/// Only performs actual dereferences on AVR targets to prevent segment faults on host.
#[derive(Clone, Copy)]
pub struct RealRegisters;

impl AvrRegisters for RealRegisters {
    fn write_tccr1a(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(TCCR1A_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn write_tccr1b(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(TCCR1B_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn write_ocr1a(&mut self, val: u16) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(OCR1A_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn write_tcnt1(&mut self, val: u16) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(TCNT1_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn read_tifr1(&self) -> u8 {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::read_volatile(TIFR1_ADDR) }
        #[cfg(not(target_arch = "avr"))]
        0
    }

    fn write_tifr1(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(TIFR1_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn write_ddrb(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(DDRB_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn read_ddrb(&self) -> u8 {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::read_volatile(DDRB_ADDR) }
        #[cfg(not(target_arch = "avr"))]
        0
    }

    fn write_portb(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(PORTB_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn read_portb(&self) -> u8 {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::read_volatile(PORTB_ADDR) }
        #[cfg(not(target_arch = "avr"))]
        0
    }

    fn write_pinb(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(PINB_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn read_pinb(&self) -> u8 {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::read_volatile(PINB_ADDR) }
        #[cfg(not(target_arch = "avr"))]
        0
    }

    fn write_ubrr0(&mut self, val: u16) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(UBRR0_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn write_ucsr0b(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(UCSR0B_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn write_ucsr0c(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(UCSR0C_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn read_ucsr0a(&self) -> u8 {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::read_volatile(UCSR0A_ADDR) }
        #[cfg(not(target_arch = "avr"))]
        0
    }

    fn write_udr0(&mut self, val: u8) {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::write_volatile(UDR0_ADDR, val); }
        #[cfg(not(target_arch = "avr"))]
        let _ = val;
    }

    fn read_udr0(&self) -> u8 {
        #[cfg(target_arch = "avr")]
        unsafe { core::ptr::read_volatile(UDR0_ADDR) }
        #[cfg(not(target_arch = "avr"))]
        0
    }
}

/// The mock register storage for testing.
#[cfg(not(target_arch = "avr"))]
pub struct MockRegistersState {
    pub tccr1a: u8,
    pub tccr1b: u8,
    pub ocr1a: u16,
    pub tcnt1: u16,
    pub tifr1: u8,
    pub sticky_tifr1: bool,
    pub ddrb: u8,
    pub portb: u8,
    pub pinb: u8,
    pub ubrr0: u16,
    pub ucsr0a: u8,
    pub ucsr0b: u8,
    pub ucsr0c: u8,
    pub udr0: u8,
}

#[cfg(not(target_arch = "avr"))]
#[derive(Clone)]
pub struct MockRegisters {
    state: std::rc::Rc<std::cell::RefCell<MockRegistersState>>,
}

#[cfg(not(target_arch = "avr"))]
impl MockRegisters {
    pub fn new() -> Self {
        Self {
            state: std::rc::Rc::new(std::cell::RefCell::new(MockRegistersState {
                tccr1a: 0,
                tccr1b: 0,
                ocr1a: 0,
                tcnt1: 0,
                tifr1: 0,
                sticky_tifr1: false,
                ddrb: 0,
                portb: 0,
                pinb: 0,
                ubrr0: 0,
                ucsr0a: 0,
                ucsr0b: 0,
                ucsr0c: 0,
                udr0: 0,
            }))
        }
    }

    pub fn get_tccr1a(&self) -> u8 { self.state.borrow().tccr1a }
    pub fn get_tccr1b(&self) -> u8 { self.state.borrow().tccr1b }
    pub fn get_ocr1a(&self) -> u16 { self.state.borrow().ocr1a }
    pub fn get_tcnt1(&self) -> u16 { self.state.borrow().tcnt1 }
    pub fn get_tifr1(&self) -> u8 { self.state.borrow().tifr1 }
    pub fn set_tifr1(&self, val: u8) { self.state.borrow_mut().tifr1 = val; }
    pub fn set_sticky_tifr1(&self, val: bool) { self.state.borrow_mut().sticky_tifr1 = val; }
    pub fn get_ddrb(&self) -> u8 { self.state.borrow().ddrb }
    pub fn get_portb(&self) -> u8 { self.state.borrow().portb }
    pub fn get_pinb(&self) -> u8 { self.state.borrow().pinb }
    pub fn get_ubrr0(&self) -> u16 { self.state.borrow().ubrr0 }
    pub fn get_ucsr0a(&self) -> u8 { self.state.borrow().ucsr0a }
    pub fn set_ucsr0a(&self, val: u8) { self.state.borrow_mut().ucsr0a = val; }
    pub fn get_ucsr0b(&self) -> u8 { self.state.borrow().ucsr0b }
    pub fn get_ucsr0c(&self) -> u8 { self.state.borrow().ucsr0c }
    pub fn get_udr0(&self) -> u8 { self.state.borrow().udr0 }
    pub fn set_udr0(&self, val: u8) { self.state.borrow_mut().udr0 = val; }
}

#[cfg(not(target_arch = "avr"))]
impl Default for MockRegisters {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(target_arch = "avr"))]
impl AvrRegisters for MockRegisters {
    fn write_tccr1a(&mut self, val: u8) { self.state.borrow_mut().tccr1a = val; }
    fn write_tccr1b(&mut self, val: u8) { self.state.borrow_mut().tccr1b = val; }
    fn write_ocr1a(&mut self, val: u16) { self.state.borrow_mut().ocr1a = val; }
    fn write_tcnt1(&mut self, val: u16) { self.state.borrow_mut().tcnt1 = val; }
    fn read_tifr1(&self) -> u8 { self.state.borrow().tifr1 }
    fn write_tifr1(&mut self, val: u8) {
        if !self.state.borrow().sticky_tifr1 {
            self.state.borrow_mut().tifr1 &= !val;
        }
    }

    fn write_ddrb(&mut self, val: u8) { self.state.borrow_mut().ddrb = val; }
    fn read_ddrb(&self) -> u8 { self.state.borrow().ddrb }
    fn write_portb(&mut self, val: u8) { self.state.borrow_mut().portb = val; }
    fn read_portb(&self) -> u8 { self.state.borrow().portb }
    fn write_pinb(&mut self, val: u8) {
        self.state.borrow_mut().portb ^= val;
        self.state.borrow_mut().pinb = val;
    }
    fn read_pinb(&self) -> u8 { self.state.borrow().pinb }

    fn write_ubrr0(&mut self, val: u16) { self.state.borrow_mut().ubrr0 = val; }
    fn write_ucsr0b(&mut self, val: u8) { self.state.borrow_mut().ucsr0b = val; }
    fn write_ucsr0c(&mut self, val: u8) { self.state.borrow_mut().ucsr0c = val; }
    fn read_ucsr0a(&self) -> u8 { self.state.borrow().ucsr0a }
    fn write_udr0(&mut self, val: u8) { self.state.borrow_mut().udr0 = val; }
    fn read_udr0(&self) -> u8 { self.state.borrow().udr0 }
}
