use core::fmt::{Error, Write};

pub struct Uart {
    base_address: usize,
}

impl Write for Uart {
	fn write_str(&mut self, out: &str) -> Result<(), Error> {
		for c in out.bytes() {
			self.put(c);
		}
		Ok(())
	}
}

impl Uart {
    const CLOCK_RATE: u32 = 22729000;
    const BUAD_RATE: u32 = 115200;

    pub fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&mut self) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            // Set LCR word len (u8)
            // 0b11 => 8 bit word len
            let lcr = 0b11;
            ptr.add(3).write_volatile(lcr);

            // Enable FIFO
            let fcr = 0b1;
            ptr.add(2).write_volatile(fcr);

            // Enable receiver interrupts
            let ri = 0b1;
            ptr.add(1).write_volatile(ri);

            // Calculate divisor for buad rate (115200)
            // The formula given in the NS16500A specification for calculating the divisor is:
            // divisor = ceil( (clock_hz) / (baud_sps x 16) )
            // For some fuckin' reason div_ceil is not stable (╯ ͠° ͟ʖ ͡°)╯┻━┻  
            let divisor: u16 = Uart::CLOCK_RATE.div_ceil(Uart::BUAD_RATE * 16) as u16;

            // Write '1' to DLAB so we can write the divisor
            ptr.add(3).write_volatile(lcr | 1 << 7);

            ptr.add(0).write_volatile(divisor as u8);
            ptr.add(1).write_volatile((divisor >> 8) as u8);

            // Reset LCR now
            ptr.add(3).write_volatile(lcr);
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u8;
        unsafe{
            if ptr.add(5).read_volatile() & 1 == 0 {
                None
            } else {
                Some(ptr.add(0).read_volatile())
            }
        }
    }

    fn put(&mut self, c: u8) {
        let ptr = self.base_address as *mut u8;
        unsafe {
            ptr.add(0).write_volatile(c);
        }
    }

}