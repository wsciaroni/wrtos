use crate::registers::AvrRegisters;
use wrtos_core::Timer;

pub const OCF1A: u8 = 1; // Output Compare A Match Flag
pub const WGM12: u8 = 3; // CTC mode bit
pub const CS11: u8 = 1;  // Prescaler clock select bit 1
pub const CS10: u8 = 0;  // Prescaler clock select bit 0

pub struct UnoTimer<R: AvrRegisters> {
    regs: R,
    overrun: bool,
}

impl<R: AvrRegisters> UnoTimer<R> {
    pub fn new(mut regs: R, ocr1a_value: u16) -> Self {
        // CTC mode, prescaler 64
        regs.write_tccr1a(0);
        regs.write_tcnt1(0);
        regs.write_ocr1a(ocr1a_value);

        // Start timer: CTC mode, Prescaler 64
        regs.write_tccr1b((1 << WGM12) | (1 << CS11) | (1 << CS10));

        // Clear flag
        regs.write_tifr1(1 << OCF1A);

        Self { regs, overrun: false }
    }

    pub fn get_regs_mut(&mut self) -> &mut R {
        &mut self.regs
    }

    pub fn release(self) -> R {
        self.regs
    }

    pub fn reset_overrun(&mut self) {
        self.overrun = false;
    }
}

impl<R: AvrRegisters> Timer for UnoTimer<R> {
    fn wait_next_tick(&mut self) {
        #[cfg(test)]
        let mut loop_count = 0;
        // Poll for OCF1A match flag
        while (self.regs.read_tifr1() & (1 << OCF1A)) == 0 {
            #[cfg(test)]
            {
                loop_count += 1;
                if loop_count > 10 {
                    break;
                }
            }
        }

        // Clear match flag by writing a 1 to it
        self.regs.write_tifr1(1 << OCF1A);

        // Check for overrun
        if (self.regs.read_tifr1() & (1 << OCF1A)) != 0 {
            self.overrun = true;
        }
    }

    fn has_overrun(&self) -> bool {
        self.overrun
    }
}
