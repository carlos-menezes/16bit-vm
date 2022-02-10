pub struct Memory {
    data: [u16; u16::MAX as usize],
}

impl Memory {
    pub fn new() -> Self {
        Self {
            data: [0u16; u16::MAX as usize],
        }
    }

    pub fn read(&self, address: u16) -> u16 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u16) {
        self.data[address as usize] = value;
    }
}
