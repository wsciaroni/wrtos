use crate::registers::AvrRegisters;

// USART register bits
pub const UDRE0: u8 = 5;
pub const RXC0: u8 = 7;
pub const TXEN0: u8 = 3;
pub const RXEN0: u8 = 4;
pub const UCSZ01: u8 = 2;
pub const UCSZ00: u8 = 1;

pub trait Serial {
    fn init(&mut self, baud_ubrr: u16);
    fn write_byte(&mut self, byte: u8);
    fn read_byte(&mut self) -> Option<u8>;
}

pub struct PollingSerial<R: AvrRegisters> {
    regs: R,
}

impl<R: AvrRegisters> PollingSerial<R> {
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

impl<R: AvrRegisters> Serial for PollingSerial<R> {
    fn init(&mut self, baud_ubrr: u16) {
        // Set baud rate
        self.regs.write_ubrr0(baud_ubrr);
        // Enable transmitter and receiver
        self.regs.write_ucsr0b((1 << TXEN0) | (1 << RXEN0));
        // Set frame format: 8 data bits, 1 stop bit
        self.regs.write_ucsr0c((1 << UCSZ01) | (1 << UCSZ00));
    }

    fn write_byte(&mut self, byte: u8) {
        #[cfg(test)]
        let mut loop_count = 0;
        // Wait for empty transmit buffer
        while (self.regs.read_ucsr0a() & (1 << UDRE0)) == 0 {
            #[cfg(test)]
            {
                loop_count += 1;
                if loop_count > 10 {
                    break;
                }
            }
        }
        self.regs.write_udr0(byte);
    }

    fn read_byte(&mut self) -> Option<u8> {
        // Check if data is received
        if (self.regs.read_ucsr0a() & (1 << RXC0)) != 0 {
            Some(self.regs.read_udr0())
        } else {
            None
        }
    }
}
