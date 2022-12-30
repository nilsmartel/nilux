/// Base address for memory mapped IO
const BASE_ADDRESS: usize = 0x1000_0000;

pub struct MMIO {
    address: usize,
}

impl core::clone::Clone for MMIO {
    fn clone(&self) -> Self {
        MMIO {
            address: self.address,
        }
    }
}
impl core::marker::Copy for MMIO {}

impl MMIO {
    unsafe fn write(self, offset: usize, value: u8) {
        let register = self.address as *mut u8;

        register.add(offset).write_volatile(value);
    }
}
