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

    pub fn read(&mut self, addr: u32) -> u8 {
        match addr {
            // BIOS ※16bit Bus
            0x00000000..=0x00003FFF => todo!("BIOS Read"),
            // EWRAM(External Work RAM) ※16bit Bus
            0x02000000..=0x0203FFFF => self.ewram[(addr & 0x3FFFF) as usize],
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => self.iram[(addr & 0x7FFF) as usize],
            // I/O
            0x04000000..=0x040003FF  => todo!("I/O Reg Read"),
            // Palette RAM ※16bit Bus
            0x05000000..=0x050003FF  => self.pram[(addr & 0x03FF) as usize],
            // VRAM ※16bit Bus
            0x06000000..=0x06017FFF  => self.vram[(addr & 0x17FFF) as usize],
            // OAM
            0x07000000..=0x070003FF  => self.oam[(addr & 0x03FF) as usize],
            // Game Pak ROM ※16bit Bus
            0x08000000..=0x09FFFFFF  => todo!("Game Pak ROM Read"),
            // Game Pak ROM Image 1 ※16bit Bus
            0x0A000000..=0x0BFFFFFF  => todo!("Game Pak ROM Image 1 Read"),
            // Game Pak ROM Image 2 ※16bit Bus
            0x0C000000..=0x0DFFFFFF  => todo!("Game Pak ROM Image 2 Read"),
            // Game Pak RAM ※8bit Bus
            0x0E000000..=0x0E00FFFF  => todo!("Game Pak RAM Read"),
            _ => panic!("[ERR] Invalid Read Addr ${:#08X}", addr),
        }
    }

    pub fn write(&mut self, addr: u32, val: u8) {
        match addr {
            // EWRAM(External Work RAM) ※16bit Bus
            0x02000000..=0x0203FFFF => self.ewram[(addr & 0x3FFFF) as usize] = val,
            // IRAM(Internal Work RAM)
            0x03000000..=0x03007FFF => self.iram[(addr & 0x7FFF) as usize] = val,
            // I/O
            0x04000000..=0x040003FF  => todo!("I/O Reg Read"),
            // Palette RAM ※16bit Bus
            0x05000000..=0x050003FF  => self.pram[(addr & 0x03FF) as usize] = val,
            // VRAM ※16bit Bus
            0x06000000..=0x06017FFF  => self.vram[(addr & 0x17FFF) as usize] = val,
            // OAM
            0x07000000..=0x070003FF  => self.oam[(addr & 0x03FF) as usize] = val,
            // Game Pak RAM ※8bit Bus
            0x0E000000..=0x0E00FFFF  => todo!("Game Pak RAM Write"),
            _ => panic!("[ERR] Invalid Write Addr ${:#08X}", addr),
        }
    }

    pub fn update(&mut self, _tick: u32) {
        // TODO
    }
}
