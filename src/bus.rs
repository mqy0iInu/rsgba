use common::*;

// EWRAM(External Work RAM)
const _EWRAM_SIZE: usize = _MEM_SIZE_256K;
// IRAM(Internal Work RAM)
const _IRAM_SIZE: usize = _MEM_SIZE_32K;
// Palette RAM
const _PRAM_SIZE: usize = _MEM_SIZE_1K;
// VRAM
const _VRAM_SIZE: usize = 96 * 1024;
// OAM
const _OAM_SIZE: usize = _MEM_SIZE_1K;

#[allow(dead_code)]
pub struct Bus {
    ewram: [u8; _EWRAM_SIZE],
    iram: [u8; _IRAM_SIZE],
    pram: [u8; _PRAM_SIZE],
    vram: [u8; _VRAM_SIZE],
    oam: [u8; _OAM_SIZE],
}

#[allow(dead_code)]
impl Bus {
    pub fn new() -> Self {
        Bus {
            ewram: [0; _EWRAM_SIZE],
            iram: [0; _IRAM_SIZE],
            pram: [0; _PRAM_SIZE],
            vram: [0; _VRAM_SIZE],
            oam: [0; _OAM_SIZE],
        }
    }

    fn read_u8(&mut self, ptr: *const u8) -> u8 {
        unsafe { *ptr }
    }

    fn read_u16(&mut self, ptr: *const u8) -> u16 {
        unsafe {
            let bytes = std::slice::from_raw_parts(ptr, 2);
            u16::from_le_bytes([bytes[0], bytes[1]])
        }
    }

    fn read_u32(&mut self, ptr: *const u8) -> u32 {
        unsafe {
            let bytes = std::slice::from_raw_parts(ptr, 4);
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        }
    }

    fn write_u8(&mut self, ptr: *mut u8, val: u8) {
        unsafe {
            *ptr = val;
        }
    }

    fn write_u16(&mut self, ptr: *mut u8, val: u16) {
        unsafe {
            let bytes = val.to_le_bytes();
            let dest = std::slice::from_raw_parts_mut(ptr, 2);
            dest.copy_from_slice(&bytes);
        }
    }

    fn write_u32(&mut self, ptr: *mut u8, val: u32) {
        unsafe {
            let bytes = val.to_le_bytes();
            let dest = std::slice::from_raw_parts_mut(ptr, 4);
            dest.copy_from_slice(&bytes);
        }
    }

    pub unsafe fn read_byte(&mut self, addr: u32) -> u8 {
        match addr {
            // BIOS
            0x00000000..=0x00003FFF => todo!("BIOS Read"),
            // EWRAM(External Work RAM)
            0x02000000..=0x0203FFFF => {
                let ptr = self.ewram.as_mut_ptr().add((addr & 0x3FFFF) as usize);
                self.read_u8(ptr)
            },
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => {
                let ptr = self.iram.as_mut_ptr().add((addr & 0x7FFF) as usize);
                self.read_u8(ptr)
            },
            // I/O
            0x04000000..=0x040003FF => todo!("I/O Reg Read"),
            // Palette RAM
            0x05000000..=0x050003FF => {
                let ptr = self.pram.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.read_u8(ptr)
            },
            // VRAM
            0x06000000..=0x06017FFF => {
                let ptr = self.vram.as_mut_ptr().add((addr & 0x17FFF) as usize);
                self.read_u8(ptr)
            },
            // OAM
            0x07000000..=0x070003FF => {
                let ptr = self.oam.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.read_u8(ptr)
            },
            // Game Pak ROM/Flash
            0x08000000..=0x09FFFFFF => todo!("Game Pak ROM/Flash Read"),
            // Game Pak ROM/Flash Image 1
            0x0A000000..=0x0BFFFFFF => todo!("Game Pak ROM/Flash Image 1 Read"),
            // Game Pak ROM/Flash Image 2
            0x0C000000..=0x0DFFFFFF => todo!("Game Pak ROM/Flash Image 2 Read"),
            // Game Pak RAM
            0x0E000000..=0x0E00FFFF => todo!("Game Pak RAM Read"),
            _ => panic!("[ERR] Invalid 8bit Bus Read Addr ${:#08X}", addr),
        }
    }

    pub unsafe fn read_hword(&mut self, addr: u32) -> u16 {
        match addr {
            // BIOS
            0x00000000..=0x00003FFF => todo!("BIOS Read"),
            // EWRAM(External Work RAM)
            0x02000000..=0x0203FFFF => {
                let ptr = self.ewram.as_mut_ptr().add((addr & 0x3FFFF) as usize);
                self.read_u16(ptr)
            },
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => {
                let ptr = self.iram.as_mut_ptr().add((addr & 0x7FFF) as usize);
                self.read_u16(ptr)
            },
            // I/O
            0x04000000..=0x040003FF => todo!("I/O Reg Read"),
            // Palette RAM
            0x05000000..=0x050003FF => {
                let ptr = self.pram.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.read_u16(ptr)
            },
            // VRAM
            0x06000000..=0x06017FFF => {
                let ptr = self.vram.as_mut_ptr().add((addr & 0x17FFF) as usize);
                self.read_u16(ptr)
            },
            // OAM
            0x07000000..=0x070003FF => {
                let ptr = self.oam.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.read_u16(ptr)
            },
            // Game Pak ROM/Flash
            0x08000000..=0x09FFFFFF => todo!("Game Pak ROM/Flash Read"),
            // Game Pak ROM/Flash Image 1
            0x0A000000..=0x0BFFFFFF => todo!("Game Pak ROM/Flash Image 1 Read"),
            // Game Pak ROM/Flash Image 2
            0x0C000000..=0x0DFFFFFF => todo!("Game Pak ROM/Flash Image 2 Read"),
            // Game Pak RAM
            0x0E000000..=0x0E00FFFF => todo!("Game Pak RAM Read"),
            _ => panic!("[ERR] Invalid 16bit Bus Read Addr ${:#08X}", addr),
        }
    }

    pub unsafe fn read_word(&mut self, addr: u32) -> u32 {
        match addr {
            // BIOS
            0x00000000..=0x00003FFF => todo!("BIOS Read"),
            // EWRAM(External Work RAM)
            0x02000000..=0x0203FFFF => {
                let ptr = self.ewram.as_mut_ptr().add((addr & 0x3FFFF) as usize);
                self.read_u32(ptr)
            },
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => {
                let ptr = self.iram.as_mut_ptr().add((addr & 0x7FFF) as usize);
                self.read_u32(ptr)
            },
            // I/O
            0x04000000..=0x040003FF => todo!("I/O Reg Read"),
            // Palette RAM
            0x05000000..=0x050003FF => {
                let ptr = self.pram.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.read_u32(ptr)
            },
            // VRAM
            0x06000000..=0x06017FFF => {
                let ptr = self.vram.as_mut_ptr().add((addr & 0x17FFF) as usize);
                self.read_u32(ptr)
            },
            // OAM
            0x07000000..=0x070003FF => {
                let ptr = self.oam.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.read_u32(ptr)
            },
            // Game Pak ROM/Flash
            0x08000000..=0x09FFFFFF => todo!("Game Pak ROM/Flash Read"),
            // Game Pak ROM/Flash Image 1
            0x0A000000..=0x0BFFFFFF => todo!("Game Pak ROM/Flash Image 1 Read"),
            // Game Pak ROM/Flash Image 2
            0x0C000000..=0x0DFFFFFF => todo!("Game Pak ROM/Flash Image 2 Read"),
            _ => panic!("[ERR] Invalid 32bit Bus Read Addr ${:#08X}", addr),
        }
    }

    pub unsafe fn write_byte(&mut self, addr: u32, val: u8) {
        match addr {
            // EWRAM(External Work RAM)
            0x02000000..=0x0203FFFF => {
                let ptr = self.ewram.as_mut_ptr().add((addr & 0x3FFFF) as usize);
                self.write_u8(ptr, val);
            },
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => {
                let ptr = self.iram.as_mut_ptr().add((addr & 0x7FFF) as usize);
                self.write_u8(ptr, val);
            },
            // I/O
            0x04000000..=0x040003FF => todo!("I/O Reg Write"),
            // Palette RAM
            0x05000000..=0x050003FF => {
                let ptr = self.pram.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.write_u8(ptr, val);
            },
            // VRAM
            0x06000000..=0x06017FFF => {
                let ptr = self.vram.as_mut_ptr().add((addr & 0x17FFF) as usize);
                self.write_u8(ptr, val);
            },
            // OAM
            0x07000000..=0x070003FF => {
                let ptr = self.oam.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.write_u8(ptr, val);
            },
            // Game Pak ROM/Flash
            0x08000000..=0x09FFFFFF => todo!("Game Pak ROM/Flash Write"),
            // Game Pak ROM/Flash Image 1
            0x0A000000..=0x0BFFFFFF => todo!("Game Pak ROM/Flash Image 1 Write"),
            // Game Pak ROM/Flash Image 2
            0x0C000000..=0x0DFFFFFF => todo!("Game Pak ROM/Flash Image 2 Write"),
            // Game Pak RAM
            0x0E000000..=0x0E00FFFF => todo!("Game Pak RAM Write"),
            _ => panic!("[ERR] Invalid 8bit Bus Write Addr ${:#08X}", addr),
        }
    }

    pub unsafe fn write_hword(&mut self, addr: u32, val: u16) {
        match addr {
            // EWRAM(External Work RAM)
            0x02000000..=0x0203FFFF => {
                let ptr = self.ewram.as_mut_ptr().add((addr & 0x3FFFF) as usize);
                self.write_u16(ptr, val);
            },
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => {
                let ptr = self.iram.as_mut_ptr().add((addr & 0x7FFF) as usize);
                self.write_u16(ptr, val);
            },
            // I/O
            0x04000000..=0x040003FF => todo!("I/O Reg Write"),
            // Palette RAM
            0x05000000..=0x050003FF => {
                let ptr = self.pram.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.write_u16(ptr, val);
            },
            // VRAM
            0x06000000..=0x06017FFF => {
                let ptr = self.vram.as_mut_ptr().add((addr & 0x17FFF) as usize);
                self.write_u16(ptr, val);
            },
            // OAM
            0x07000000..=0x070003FF => {
                let ptr = self.oam.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.write_u16(ptr, val);
            },
            // Game Pak ROM/Flash
            0x08000000..=0x09FFFFFF => todo!("Game Pak ROM/Flash Write"),
            // Game Pak ROM/Flash Image 1
            0x0A000000..=0x0BFFFFFF => todo!("Game Pak ROM/Flash Image 1 Write"),
            // Game Pak ROM/Flash Image 2
            0x0C000000..=0x0DFFFFFF => todo!("Game Pak ROM/Flash Image 2 Write"),
            _ => panic!("[ERR] Invalid 16bit Bus Write Addr ${:#08X}", addr),
        }
    }

    pub unsafe fn write_word(&mut self, addr: u32, val: u32) {
        match addr {
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => {
                let ptr = self.iram.as_mut_ptr().add((addr & 0x7FFF) as usize);
                self.write_u32(ptr, val);
            },
            // I/O
            0x04000000..=0x040003FF => todo!("I/O Reg Write"),
            // OAM
            0x07000000..=0x070003FF => {
                let ptr = self.oam.as_mut_ptr().add((addr & 0x03FF) as usize);
                self.write_u32(ptr, val);
            },
            _ => panic!("[ERR] Invalid 32bit Bus Write Addr ${:#08X}", addr),
        }
    }

    pub fn update(&mut self, _tick: u32) {
        // TODO
    }
}
