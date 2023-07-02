use thumb_op::*;
use arm_op::*;
use bus::*;
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct PSR: u32 {
        const N = 1 << 31;              // Negative Flag
        const Z = 1 << 30;              // Zero Flag
        const C = 1 << 29;              // Carry Flag
        const V = 1 << 28;              // Overflow Flag
        const I = 1 << 7;               // IRQ disable
        const F = 1 << 6;               // FIQ disable
        const T = 1 << 5;               // Thumb state indicator
        const MODE_BITS = 0b0000_1111;  // Mode Bits
    }
}

// ARM7TDMI レジスタ構造体
#[allow(dead_code)]
pub struct Register {
    pub r: [u32; 12],     // 汎用レジスタ R0~R12
    pub sp: u32,          // R13(SP)
    pub lr: u32,          // R14(LR)
    pub pc: u32,          // R15(PC)
    pub cpsr: PSR,        // 現在ステータスレジスタ
    pub spsr: [PSR; 5],   // 保存ステータスレジスタ(FIQ,SVC,Abort,IRQ,Undefined)
}

impl Register {
    pub fn new() -> Self {
        Register {
            r: [0; 12],
            sp: 0,
            lr: 0,
            pc: 0,
            cpsr: PSR::empty(),
            spsr: [PSR::empty(); 5],
        }
    }

    #[allow(dead_code)]
    // (DEBUG) Trace Debug用　レジスタ名取得
    fn get_reg_str(idx: u8) -> String {
        match idx {
            0 => String::from("R0"),
            1 => String::from("R1"),
            2 => String::from("R2"),
            3 => String::from("R3"),
            4 => String::from("R4"),
            5 => String::from("R5"),
            6 => String::from("R6"),
            7 => String::from("R7"),
            8 => String::from("R8"),
            9 => String::from("R9"),
            10 => String::from("R10"),
            11 => String::from("R11"),
            12 => String::from("R12"),
            13 => String::from("R13(SP)"),
            14 => String::from("R14(LR)"),
            15 => String::from("R15(PC)"),
            16 => String::from("CPSR"),
            17 => String::from("SPSR"),
            _ => panic!("Invalid get_reg_str() arg: {}", idx),
        }
    }
}

bitflags! {
    #[derive(Clone, Copy)]
    struct Mode: u8 {
        const USER = 0b0001_0000;  // User Mode
        const FIQ  = 0b0001_0001;  // FIQ Mode
        const IRQ  = 0b0001_0010;  // IRQ Mode
        const SVC  = 0b0001_0011;  // Supervisor Mode
        const ABT  = 0b0001_0101;  // Abort Mode
        const UDF  = 0b0001_0111;  // Undefined Mode
        const SYS  = 0b0001_1111;  // System Mode
    }
}

#[allow(dead_code)]
pub struct CPU {
    pub reg: Register,
    bus: Bus,
    mode: Mode,
    tick: u32,
}

#[allow(dead_code)]
impl CPU {
    pub fn new() -> Self {
        let mut _mode = Mode::empty();
        _mode.insert(Mode::USER);

        CPU {
            reg: Register::new(),
            bus: Bus::new(),
            mode: _mode,
            tick: 0,
        }
    }

    // Thumb命令(16bit)
    fn op_thumb(&mut self) {
        unsafe {
            // Fetch Thumb
            let _op: u16 = self.bus.read_hword(self.reg.pc);
            // Decode Thumb
            thumb_op_decode(self,_op);
            // Exec Thumb
            thumb_op_exec(self, _op);
        }
    }

    // ARM命令(32bit)
    fn op_arm(&mut self) {
        unsafe {
            // Fetch ARM
            let _op: u32 = ((self.bus.read_hword(self.reg.pc) as u32) << 16) |
                            (self.bus.read_hword(self.reg.pc + 1) as u32);
            // Decode ARM
            arm_op_decode(self, _op);
            // Exec ARM
            arm_op_exec(self, _op);
        }
    }

    fn decode_and_exec(&mut self, _op: u32) {
        // TODO
        trace!("OP: {:#04}", _op);
    }

    pub fn proc(&mut self) {
        // Fetch & Decode & Execute
        match self.reg.cpsr.contains(PSR::T) {
            false => self.op_thumb(),
            true => self.op_arm(),
        }

        self.bus.update(self.tick);
    }
}
