// =========================================================================
// [Common Define]
// =========================================================================
pub const _BIT_0:  u32 = 0x00000001;
pub const _BIT_1:  u32 = 0x00000002;
pub const _BIT_2:  u32 = 0x00000004;
pub const _BIT_3:  u32 = 0x00000008;
pub const _BIT_4:  u32 = 0x00000010;
pub const _BIT_5:  u32 = 0x00000020;
pub const _BIT_6:  u32 = 0x00000040;
pub const _BIT_7:  u32 = 0x00000080;
pub const _BIT_8:  u32 = 0x00000100;
pub const _BIT_9:  u32 = 0x00000200;
pub const _BIT_10: u32 = 0x00000400;
pub const _BIT_11: u32 = 0x00000800;
pub const _BIT_12: u32 = 0x00001000;
pub const _BIT_13: u32 = 0x00002000;
pub const _BIT_14: u32 = 0x00004000;
pub const _BIT_15: u32 = 0x00008000;
pub const _BIT_16: u32 = 0x00010000;
pub const _BIT_17: u32 = 0x00020000;
pub const _BIT_18: u32 = 0x00040000;
pub const _BIT_19: u32 = 0x00080000;
pub const _BIT_20: u32 = 0x00100000;
pub const _BIT_21: u32 = 0x00200000;
pub const _BIT_22: u32 = 0x00400000;
pub const _BIT_23: u32 = 0x00800000;
pub const _BIT_24: u32 = 0x01000000;
pub const _BIT_25: u32 = 0x02000000;
pub const _BIT_26: u32 = 0x04000000;
pub const _BIT_27: u32 = 0x08000000;
pub const _BIT_28: u32 = 0x10000000;
pub const _BIT_29: u32 = 0x20000000;
pub const _BIT_30: u32 = 0x40000000;
pub const _BIT_31: u32 = 0x80000000;

pub const _MEM_SIZE_1K:   usize =   1 * 1024;
pub const _MEM_SIZE_2K:   usize =   2 * 1024;
pub const _MEM_SIZE_4K:   usize =   4 * 1024;
pub const _MEM_SIZE_8K:   usize =   8 * 1024;
pub const _MEM_SIZE_16K:  usize =  16 * 1024;
pub const _MEM_SIZE_32K:  usize =  32 * 1024;
pub const _MEM_SIZE_64K:  usize =  64 * 1024;
pub const _MEM_SIZE_128K: usize = 128 * 1024;
pub const _MEM_SIZE_256K: usize = 256 * 1024;
pub const _MEM_SIZE_512K: usize = 512 * 1024;

pub const _MBC_0: u8 = 0;
pub const _MBC_1: u8 = 1;
pub const _MBC_2: u8 = 2;
pub const _MBC_3: u8 = 3;
pub const _MBC_4: u8 = 4;
pub const _MBC_5: u8 = 5;

pub const _SCREEN_W: u8 = 240;
pub const _SCREEN_H: u8 = 160;

// =========================================================================
pub trait IO {
    fn write(&mut self, addr: u16, val: u8);
    fn read(&mut self, addr: u16) -> u8;
    fn update(&mut self, tick: u8);
}