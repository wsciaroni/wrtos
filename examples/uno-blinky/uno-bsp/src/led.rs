use crate::registers::AvrRegisters;

pub const PB5: u8 = 5;

pub trait Led {
    fn init(&mut self);
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn toggle(&mut self);
    fn is_on(&self) -> bool;
}

pub struct PinLed<R: AvrRegisters> {
    regs: R,
}

impl<R: AvrRegisters> PinLed<R> {
    pub fn new(regs: R) -> Self {
        Self { regs }
    }

    pub fn get_regs_mut(&mut self) -> &mut R {
        &mut self.regs
    }

    pub fn release(self) -> R {
        self.regs
    }
}

impl<R: AvrRegisters> Led for PinLed<R> {
    fn init(&mut self) {
        let ddr = self.regs.read_ddrb();
        self.regs.write_ddrb(ddr | (1 << PB5));
    }

    fn set_high(&mut self) {
        let port = self.regs.read_portb();
        self.regs.write_portb(port | (1 << PB5));
    }

    fn set_low(&mut self) {
        let port = self.regs.read_portb();
        self.regs.write_portb(port & !(1 << PB5));
    }

    fn toggle(&mut self) {
        self.regs.write_pinb(1 << PB5);
    }

    fn is_on(&self) -> bool {
        (self.regs.read_portb() & (1 << PB5)) != 0
    }
}
