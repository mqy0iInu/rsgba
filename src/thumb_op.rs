// use bitflags::Flags;
use cpu::*;

#[derive(Debug, PartialEq)]
pub enum ThumbFormat {
    Format01, // Format01 ... Move shifted register
    Format02, // Format02 ... Add and subtract
    Format03, // Format03 ... Move, compare, add, and subtract immediate
    Format04, // Format04 ... ALU operation
    Format05, // Format05 ... High register operations and branch exchange
    Format06, // Format06 ... PC-relative load
    Format07, // Format07 ... Load and store with relative offset
    Format08, // Format08 ... Load and store sign-extended byte and halfword
    Format09, // Format09 ... Load and store with immediate offset
    Format10, // Format10 ... Load and store halfword
    Format11, // Format11 ... SP-relative load and store
    Format12, // Format12 ... Load address
    Format13, // Format13 ... Add offset to stack pointer
    Format14, // Format14 ... Push and pop registers
    Format15, // Format15 ... Multiple load and store
    Format16, // Format16 ... Conditional branch
    Format17, // Format17 ... Software interrupt
    Format18, // Format18 ... Unconditional branch
    Format19, // Format19 ... Long branch with link
}

#[derive(Debug, PartialEq)]
pub enum ThumbInstruction {
    // Format01: シフトレジスタ付きレジスタ移動
    MoveShiftedRegister(u8, i8, u8, u8), // |Op|Offset5|Rs|Rd|

    // Format02: 加算/減算
    AddSubtract(u8, i8, u8, u8), // |Op|Rn/Offset3|Rs|Rd|

    // Format03: レジスタ移動/比較/加算/減算（即値）
    MoveCompareAddSubtractImmediate(u8, u8, i8), // |Op|Rd|Offset8|

    // Format04: ALU演算
    ALUOperation(u8, u8, u8), // |Op|Rs|Rd|

    // Format05: 高位レジスタ操作/分岐交換
    HighRegisterOperationsBranchExchange(u8, u8, u8, u8, u8), // |Op|H1|H2|Rs/Hs|RdHd|

    // Format06: PC相対ロード
    PCRelativeLoad(u8, u8), // |Rd|Word8|

    // Format07: 相対オフセットを使用したロード/ストア
    LoadStoreWithRelativeOffset(u8, u8, u8, u8, u8), // |L|B|Ro|Rb|Rd|

    // Format08: 符号拡張されたバイト/ハーフワードのロード/ストア
    LoadStoreSignExtendedByteHalfword(u8, u8, u8, u8, u8), // |H|S|Ro|Rb|Rd|

    // Format09: 即値オフセットを使用したロード/ストア
    LoadStoreWithImmediateOffset(u8, u8, i8, u8, u8), // |B|L|Offset5|Rb|Rd|

    // Format10: ハーフワードのロード/ストア
    LoadStoreHalfword(u8, i8, u8, u8), // |L|Offset5|Rb|Rd|

    // Format11: SP相対ロード/ストア
    SPRelativeLoadStore(u8, u8, u8), // |L|Rb|Word8|

    // Format12: アドレスのロード
    LoadAddress(u8, u8, u8), // |SP|Rb|Word8|

    // Format13: スタックポインタへのオフセットの追加
    AddOffsetToStackPointer(u8, u8), // |S|SWord7|

    // Format14: レジスタのプッシュ/ポップ
    PushPopRegisters(u8, u8, u8), // |L|R|Rlist|

    // Format15: 複数のロード/ストア
    MultipleLoadStore(u8, u8, u8), // |L|Rb|Rlist|

    // Format16: 条件付き分岐
    ConditionalBranch(u8, i8), // |Cond|Softset8|

    // Format17: ソフトウェア割り込み
    SoftwareInterrupt(u8), // |Value8|

    // Format18: 無条件分岐
    UnconditionalBranch(i16), // |Offset11|

    // Format19: リンク付きロング分岐
    LongBranchWithLink(u8, i16), // |H|Offset|
}

fn decode_format01(instruction: u16) -> ThumbInstruction {
    let op: u8 = ((instruction & 0b0001_1000_0000_0000) >> 11) as u8;
    let offset5: i8 = ((instruction & 0b0000_0111_1100_0000) >> 6) as i8;
    let rs: u8 = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rd: u8 = (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::MoveShiftedRegister(op, offset5, rs, rd)
}

fn decode_format02(instruction: u16) -> ThumbInstruction {
    let op: u8 = ((instruction & 0b0000_0010_0000_0000) >> 9) as u8;
    let rn: i8 = ((instruction & 0b0000_0001_1100_0000) >> 6) as i8;
    let rs: u8 = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rd: u8 = (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::AddSubtract(op, rn, rs, rd)
}

fn decode_format03(instruction: u16) -> ThumbInstruction {
    let op: u8 = ((instruction & 0b0001_1000_0000_0000) >> 11) as u8;
    let rd: u8 = ((instruction & 0b0000_0111_0000_0000) >> 8) as u8;
    let offset: i8 = (instruction & 0b0000_0000_1111_1111) as i8;
    ThumbInstruction::MoveCompareAddSubtractImmediate(op, rd, offset)
}

fn decode_format04(instruction: u16) -> ThumbInstruction {
    let op: u8 = ((instruction & 0b0000_0011_1100_0000) >> 6) as u8;
    let rs: u8 = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rd: u8 = (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::ALUOperation(op, rs, rd)
}

fn decode_format05(instruction: u16) -> ThumbInstruction {
    let op: u8 = ((instruction & 0b0000_0011_0000_0000) >> 8) as u8;
    let h1: u8 = ((instruction & 0b0000_0000_1000_0000) >> 7) as u8;
    let h2: u8 = ((instruction & 0b0000_0000_0100_0000) >> 6) as u8;
    let rs: u8 = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rd: u8 = (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::HighRegisterOperationsBranchExchange(op, h1, h2, rs, rd)
}

fn decode_format06(instruction: u16) -> ThumbInstruction {
    let rd: u8 = ((instruction & 0b0000_0111_0000_0000) >> 8) as u8;
    let word8: u8 = (instruction & 0b0000_0000_1111_1111) as u8;
    ThumbInstruction::PCRelativeLoad(rd, word8)
}

fn decode_format07(instruction: u16) -> ThumbInstruction {
    let l: u8 = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let b: u8 = ((instruction & 0b0000_0100_0000_0000) >> 10) as u8;
    let ro: u8 = ((instruction & 0b0000_0001_1100_0000) >> 6) as u8;
    let rd: u8 = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rb: u8 = (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::LoadStoreWithRelativeOffset(l, b, ro, rd, rb)
}

fn decode_format08(instruction: u16) -> ThumbInstruction {
    let h: u8 = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let s: u8 = ((instruction & 0b0000_0100_0000_0000) >> 10) as u8;
    let ro: u8 = ((instruction & 0b0000_0001_1100_0000) >> 6) as u8;
    let rd: u8 = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rb: u8 = (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::LoadStoreSignExtendedByteHalfword(h, s, ro, rd, rb)
}

fn decode_format09(instruction: u16) -> ThumbInstruction {
    let b: u8      = ((instruction & 0b0001_0000_0000_0000) >> 12) as u8;
    let l: u8      = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let offset: i8 = ((instruction & 0b0000_0111_1100_0000) >> 6) as i8;
    let rd: u8     = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rb: u8     =  (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::LoadStoreWithImmediateOffset(b, l, offset, rd, rb)
}

fn decode_format10(instruction: u16) -> ThumbInstruction {
    let l: u8      = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let offset: i8 = ((instruction & 0b0000_0111_1100_0000) >> 6) as i8;
    let rd: u8     = ((instruction & 0b0000_0000_0011_1000) >> 3) as u8;
    let rb: u8     =  (instruction & 0b0000_0000_0000_0111) as u8;
    ThumbInstruction::LoadStoreHalfword(l, offset, rd, rb)
}

fn decode_format11(instruction: u16) -> ThumbInstruction {
    let l: u8      = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let rd: u8     = ((instruction & 0b0000_0111_0000_0000) >> 8) as u8;
    let word8: u8  =  (instruction & 0b0000_0000_1111_1111) as u8;
    ThumbInstruction::SPRelativeLoadStore(l, rd, word8)
}

fn decode_format12(instruction: u16) -> ThumbInstruction {
    let sp: u8     = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let rd: u8     = ((instruction & 0b0000_0111_0000_0000) >> 8) as u8;
    let word8: u8  =  (instruction & 0b0000_0000_1111_1111) as u8;
    ThumbInstruction::LoadAddress(sp, rd, word8)
}

fn decode_format13(instruction: u16) -> ThumbInstruction {
    let s: u8       = ((instruction & 0b0000_0000_1000_0000) >> 7) as u8;
    let sword7: u8  =  (instruction & 0b0000_0000_0111_1111) as u8;
    ThumbInstruction::AddOffsetToStackPointer(s, sword7)
}

fn decode_format14(instruction: u16) -> ThumbInstruction {
    let l: u8      = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let r: u8      = ((instruction & 0b0000_0001_0000_0000) >> 8) as u8;
    let rlist: u8  =  (instruction & 0b0000_0000_1111_1111) as u8;
    ThumbInstruction::PushPopRegisters(l, r, rlist)
}

fn decode_format15(instruction: u16) -> ThumbInstruction {
    let l: u8      = ((instruction & 0b0000_1000_0000_0000) >> 11) as u8;
    let rb: u8     = ((instruction & 0b0000_0111_0000_0000) >> 8) as u8;
    let rlist: u8  =  (instruction & 0b0000_0000_1111_1111) as u8;
    ThumbInstruction::MultipleLoadStore(l, rb, rlist)
}

fn decode_format16(instruction: u16) -> ThumbInstruction {
    let cond: u8     = ((instruction & 0b0000_1111_0000_0000) >> 8) as u8;
    let softset8: i8  =  (instruction & 0b0000_0000_1111_1111) as i8;
    ThumbInstruction::ConditionalBranch(cond, softset8)
}

fn decode_format17(instruction: u16) -> ThumbInstruction {
    let value8: u8 = (instruction & 0b0000_0000_1111_1111) as u8;
    ThumbInstruction::SoftwareInterrupt(value8)
}

fn decode_format18(instruction: u16) -> ThumbInstruction {
    let offset11: i16 = (instruction & 0b0000_0111_1111_1111) as i16;
    ThumbInstruction::UnconditionalBranch(offset11)
}

fn decode_format19(instruction: u16) -> ThumbInstruction {
    let h: u8      = ((instruction & 0b0000_1000_0000_0000) >> 8) as u8;
    let offset: i16 = (instruction & 0b0000_0111_1111_1111) as i16;
    ThumbInstruction::LongBranchWithLink(h, offset)
}

pub fn thumb_format_decode(instruction: u16) -> (ThumbFormat, ThumbInstruction) {
    // Thumb命令のフォーマット判別（※ビットが立つ範囲で判定する）
    // FIXME Format1,2、Format16,17で最後のビット範囲が被る
    // ※Format16のcondが0x0FはFormat17なので0b1101_1110_1111_1111までに変更した（あってるかは不明）
    let format: ThumbFormat = match instruction {
        0b0000_0000_0000_0000..=0b0001_1111_1111_1111 => ThumbFormat::Format01,
        0b0001_1100_0000_0000..=0b0001_1111_1111_1111 => ThumbFormat::Format02,
        0b0010_0000_0000_0000..=0b0011_1111_1111_1111 => ThumbFormat::Format03,
        0b0100_0000_0000_0000..=0b0100_0011_1111_1111 => ThumbFormat::Format04,
        0b0100_0100_0000_0000..=0b0100_0111_1111_1111 => ThumbFormat::Format05,
        0b0100_1000_0000_0000..=0b0100_1111_1111_1111 => ThumbFormat::Format06,
        0b0101_0000_0000_0000..=0b0101_1101_1111_1111 => ThumbFormat::Format07,
        0b0101_0000_0000_0000..=0b0101_1111_1111_1111 => ThumbFormat::Format08,
        0b0110_0000_0000_0000..=0b0111_1111_1111_1111 => ThumbFormat::Format09,
        0b1000_0000_0000_0000..=0b1000_1111_1111_1111 => ThumbFormat::Format10,
        0b1001_0000_0000_0000..=0b1001_1111_1111_1111 => ThumbFormat::Format11,
        0b1010_0000_0000_0000..=0b1010_1111_1111_1111 => ThumbFormat::Format12,
        0b1011_0000_0000_0000..=0b1011_0000_1111_1111 => ThumbFormat::Format13,
        0b1011_0100_0000_0000..=0b1011_1101_1111_1111 => ThumbFormat::Format14,
        0b1100_0000_0000_0000..=0b1100_1111_1111_1111 => ThumbFormat::Format15,
        0b1101_0000_0000_0000..=0b1101_1110_1111_1111 => ThumbFormat::Format16,
        0b1101_1111_0000_0000..=0b1101_1111_1111_1111 => ThumbFormat::Format17,
        0b1110_0000_0000_0000..=0b1110_0111_1111_1111 => ThumbFormat::Format18,
        0b1111_0000_0000_0000..=0b1111_1111_1111_1111 => ThumbFormat::Format19,
        _ => panic!("Unknown Thumb instruction format"),
    };

    // Thumb命令のフォーマットに応じて命令をでコード
    match format {
        ThumbFormat::Format01 => (format, decode_format01(instruction)),
        ThumbFormat::Format02 => (format, decode_format02(instruction)),
        ThumbFormat::Format03 => (format, decode_format03(instruction)),
        ThumbFormat::Format04 => (format, decode_format04(instruction)),
        ThumbFormat::Format05 => (format, decode_format05(instruction)),
        ThumbFormat::Format06 => (format, decode_format06(instruction)),
        ThumbFormat::Format07 => (format, decode_format07(instruction)),
        ThumbFormat::Format08 => (format, decode_format08(instruction)),
        ThumbFormat::Format09 => (format, decode_format09(instruction)),
        ThumbFormat::Format10 => (format, decode_format10(instruction)),
        ThumbFormat::Format11 => (format, decode_format11(instruction)),
        ThumbFormat::Format12 => (format, decode_format12(instruction)),
        ThumbFormat::Format13 => (format, decode_format13(instruction)),
        ThumbFormat::Format14 => (format, decode_format14(instruction)),
        ThumbFormat::Format15 => (format, decode_format15(instruction)),
        ThumbFormat::Format16 => (format, decode_format16(instruction)),
        ThumbFormat::Format17 => (format, decode_format17(instruction)),
        ThumbFormat::Format18 => (format, decode_format18(instruction)),
        ThumbFormat::Format19 => (format, decode_format19(instruction)),
    }
}

// ADD Rd, Rs
fn add(_cpu: &mut CPU, rs: u8, rd: u8) {
    let _ret: u32 = _cpu.reg.r[rd as usize].wrapping_add(_cpu.reg.r[rs as usize]);
    _cpu.reg.r[rd as usize] = _ret;
    _cpu.psr_op_update(_ret, false, false);
}

// SUB Rd, Rs
fn sub(_cpu: &mut CPU, rs: u8, rd: u8) {
    let _ret: u32 = _cpu.reg.r[rd as usize].wrapping_sub(_cpu.reg.r[rs as usize]);
    _cpu.reg.r[rd as usize] = _ret;
    _cpu.psr_op_update(_ret, true, false);
}

// CMP Rd, Rs
fn cmp(_cpu: &mut CPU, rs: u8, rd: u8) {
    let _ret: u32 = _cpu.reg.r[rd as usize].wrapping_sub(_cpu.reg.r[rs as usize]);
    _cpu.psr_op_update(_ret, true, false);
}

// fn mov(_cpu: &mut CPU, _op: ThumbInstruction)
// {
//     // TODO
// }

// AND Rd, Rs
fn and(_cpu: &mut CPU, rs: u8, rd: u8)
{
    let _ret: u32 = _cpu.reg.r[rd as usize] & _cpu.reg.r[rs as usize];
    _cpu.reg.r[rd as usize] = _ret;
    _cpu.psr_op_update(_ret, false, false);
}

// EOR Rd, Rs
fn eor(_cpu: &mut CPU, rs: u8, rd: u8)
{
    let _ret: u32 = _cpu.reg.r[rd as usize] ^ _cpu.reg.r[rs as usize];
    _cpu.reg.r[rd as usize] = _ret;
    _cpu.psr_op_update(_ret, false, false);
}

// ORR Rd, Rs
fn orr(_cpu: &mut CPU, rs: u8, rd: u8)
{
    let _ret: u32 = _cpu.reg.r[rd as usize] | _cpu.reg.r[rs as usize];
    _cpu.reg.r[rd as usize] = _ret;
    _cpu.psr_op_update(_ret, false, false);
}

// BIC Rd, Rs
fn bic(_cpu: &mut CPU, rs: u8, rd: u8)
{
    let _ret: u32 = _cpu.reg.r[rd as usize] & (!_cpu.reg.r[rs as usize]);
    _cpu.reg.r[rd as usize] = _ret;
    _cpu.psr_op_update(_ret, false, false);
}

// MVN Rd, Rs
fn mvn(_cpu: &mut CPU, rs: u8, rd: u8)
{
    let _ret = !_cpu.reg.r[rs as usize];
    _cpu.reg.r[rd as usize] = _ret;
    _cpu.psr_op_update(_ret, false, false);
}

// TST Rd, Rs
fn tst(_cpu: &mut CPU, rs: u8, rd: u8) {
    let _ret: u32 = _cpu.reg.r[rd as usize] & _cpu.reg.r[rs as usize];
    _cpu.psr_op_update(_ret, false, false);
}

fn beq(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::Z) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bne(_cpu: &mut CPU, softset8: i8) -> bool {
    if !_cpu.reg.cpsr.contains(PSR::Z) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bcs(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::C) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bcc(_cpu: &mut CPU, softset8: i8) -> bool {
    if !_cpu.reg.cpsr.contains(PSR::C) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bmi(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::N) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bpl(_cpu: &mut CPU, softset8: i8) -> bool {
    if !_cpu.reg.cpsr.contains(PSR::N) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bvs(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::V) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bvc(_cpu: &mut CPU, softset8: i8) -> bool {
    if !_cpu.reg.cpsr.contains(PSR::V) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bhi(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::C) && !_cpu.reg.cpsr.contains(PSR::Z) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bls(_cpu: &mut CPU, softset8: i8) -> bool {
    if !_cpu.reg.cpsr.contains(PSR::C) || _cpu.reg.cpsr.contains(PSR::Z) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bge(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::N) == _cpu.reg.cpsr.contains(PSR::V) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn blt(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::N) != _cpu.reg.cpsr.contains(PSR::V) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn bgt(_cpu: &mut CPU, softset8: i8) -> bool {
    if !_cpu.reg.cpsr.contains(PSR::Z) && (_cpu.reg.cpsr.contains(PSR::N) == _cpu.reg.cpsr.contains(PSR::V)) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

fn ble(_cpu: &mut CPU, softset8: i8) -> bool {
    if _cpu.reg.cpsr.contains(PSR::Z) || (_cpu.reg.cpsr.contains(PSR::N) != _cpu.reg.cpsr.contains(PSR::V)) {
        let offset: i32 = softset8 as i32;
        _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
        true
    } else {
        false
    }
}

// SWI Imm8bit
fn swi(_cpu: &mut CPU, _val: u8)
{
    // PCにImm8bit
    _cpu.reg.pc = _val as u32;
    // modeをSVCに
    _cpu.reg.cpsr.insert(PSR::MODE_SVC);
    // LRを+=2
    _cpu.reg.lr += 2;
}

fn b(_cpu: &mut CPU, softset11: i16) {
    let offset: i32 = softset11 as i32;
    _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
}

fn bl(_cpu: &mut CPU, softset11: i16) {
    let offset: i32 = softset11 as i32;
    _cpu.reg.lr = _cpu.reg.pc.wrapping_add(2);
    _cpu.reg.pc = _cpu.reg.pc.wrapping_add((offset << 1) as u32 + 2);
}

fn exec_op_format01(_cpu: &mut CPU, _op: ThumbInstruction) {
    if let ThumbInstruction::MoveShiftedRegister(op, offset, rs, rd) = _op {
        trace!("Format01: MoveShiftedRegister - Op: {}, Offset: {}, Rs: {}, Rd: {}", op, offset, rs, rd);
        // TODO
    }
}

fn exec_op_format02(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::AddSubtract(op, offset, rs, rd) = _op {
        trace!("Format02: AddSubtract - Op: {}, Offset: {}, Rs: {}, Rd: {}", op, offset, rs, rd);
    }
}

fn exec_op_format03(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::MoveCompareAddSubtractImmediate(op, rd, offset) = _op {
        trace!("Format03: MoveCompareAddSubtractImmediate - Op: {}, Rd: {}, Offset: {}", op, rd, offset);
    }
}

fn exec_op_format04(_cpu: &mut CPU, _op: ThumbInstruction) {
    if let ThumbInstruction::ALUOperation(op, rs, rd) = _op {
        trace!("Format04: ALUOperation - Op: {}, Rs: {}, Rd: {}", op, rs, rd);
        match op {
            0b0000 => and(_cpu, rs, rd),
            0b0001 => eor(_cpu, rs, rd),
            0b0010 => orr(_cpu, rs, rd),
            0b0011 => bic(_cpu, rs, rd),
            0b0100 => mvn(_cpu, rs, rd),
            0b0101 => tst(_cpu, rs, rd),
            _ => panic!("Unknown Format04(ALU Op) Execute"),
        }
        _cpu.tick += 1; // Cycle += 1S
    }
}

fn exec_op_format05(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::HighRegisterOperationsBranchExchange(op, h1, h2, rs_hs, rd_hd) = _op {
        trace!("Format05: HighRegisterOperationsBranchExchange - Op: {}, H1: {}, H2: {}, Rs/Hs: {}, Rd/Hd: {}", op, h1, h2, rs_hs, rd_hd);
    }
}

fn exec_op_format06(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::PCRelativeLoad(rd, word8) = _op {
        trace!("Format06: PCRelativeLoad - Rd: {}, Word8: {}", rd, word8);
    }
}

fn exec_op_format07(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::LoadStoreWithRelativeOffset(l, b, ro, rb, rd) = _op {
        trace!("Format07: LoadStoreWithRelativeOffset - L: {}, B: {}, Ro: {}, Rb: {}, Rd: {}", l, b, ro, rb, rd);
    }
}

fn exec_op_format08(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::LoadStoreSignExtendedByteHalfword(h, s, ro, rb, rd) = _op {
        trace!("Format08: LoadStoreSignExtendedByteHalfword - H: {}, S: {}, Ro: {}, Rb: {}, Rd: {}", h, s, ro, rb, rd);
    }
}

fn exec_op_format09(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::LoadStoreWithImmediateOffset(b, l, offset5, rb, rd) = _op {
        trace!("Format09: LoadStoreWithImmediateOffset - B: {}, L: {}, Offset5: {}, Rb: {}, Rd: {}", b, l, offset5, rb, rd);
    }
}

fn exec_op_format10(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::LoadStoreHalfword(l, offset5, rb, rd) = _op {
        trace!("Format10: LoadStoreHalfword - L: {}, Offset5: {}, Rb: {}, Rd: {}", l, offset5, rb, rd);
    }
}

fn exec_op_format11(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::SPRelativeLoadStore(l, rb, word8) = _op {
        trace!("Format11: SPRelativeLoadStore - L: {}, Rb: {}, Word8: {}", l, rb, word8);
    }
}

fn exec_op_format12(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::LoadAddress(sp, rb, word8) = _op {
        trace!("Format12: LoadAddress - SP: {}, Rb: {}, Word8: {}", sp, rb, word8);
    }
}

fn exec_op_format13(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::AddOffsetToStackPointer(s, sword7) = _op {
        trace!("Format13: AddOffsetToStackPointer - S: {}, SWord7: {}", s, sword7);
    }
}

fn exec_op_format14(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::PushPopRegisters(l, r, rlist) = _op {
        trace!("Format14: PushPopRegisters - L: {}, R: {}, Rlist: {}", l, r, rlist);
    }
}

fn exec_op_format15(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::MultipleLoadStore(l, rb, rlist) = _op {
        trace!("Format15: MultipleLoadStore - L: {}, Rb: {}, Rlist: {}", l, rb, rlist);
    }
}

fn exec_op_format16(_cpu: &mut CPU, _op: ThumbInstruction) {
    let mut _ret = false;
    if let ThumbInstruction::ConditionalBranch(cond, softset8) = _op {
        trace!("Format16: ConditionalBranch - Cond: {}, Softset8: {}", cond, softset8);
        match cond {
            0x00 => _ret = beq(_cpu, softset8),
            0x01 => _ret = bne(_cpu, softset8),
            0x02 => _ret = bcs(_cpu, softset8),
            0x03 => _ret = bcc(_cpu, softset8),
            0x04 => _ret = bmi(_cpu, softset8),
            0x05 => _ret = bpl(_cpu, softset8),
            0x06 => _ret = bvs(_cpu, softset8),
            0x07 => _ret = bvc(_cpu, softset8),
            0x08 => _ret = bhi(_cpu, softset8),
            0x09 => _ret = bls(_cpu, softset8),
            0x0A => _ret = bge(_cpu, softset8),
            0x0B => _ret = blt(_cpu, softset8),
            0x0C => _ret = bgt(_cpu, softset8),
            0x0D => _ret = ble(_cpu, softset8),
            _ => panic!("Unknown Format16(ALU Op) Execute"),
        }
        if _ret != false {
            _cpu.tick += 3; // Cycle += 2S+1N
        }else{
            _cpu.tick += 1; // Cycle += 1S
        }
    }
}

fn exec_op_format17(_cpu: &mut CPU, _op: ThumbInstruction) {
    if let ThumbInstruction::SoftwareInterrupt(value8) = _op {
        trace!("Format17: SoftwareInterrupt - Value8: {}", value8);
        swi(_cpu, value8);
        _cpu.tick += 3; // Cycle += 2S+1N
    }
}

fn exec_op_format18(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::UnconditionalBranch(offset11) = _op {
        trace!("Format18: UnconditionalBranch - Offset11: {}", offset11);
    }
}

fn exec_op_format19(_cpu: &mut CPU, _op: ThumbInstruction) {
    // TODO
    if let ThumbInstruction::LongBranchWithLink(h, offset) = _op {
        trace!("Format19: LongBranchWithLink - H: {}, Offset: {}", h, offset);
    }
}

// ARM7TDMI Thumb命令 デコード
pub fn thumb_op_decode(_cpu: &mut CPU, _instruction: u16) -> (ThumbFormat, ThumbInstruction) {
    let _op_format = thumb_format_decode(_instruction);
    _op_format
}

// ARM7TDMI Thumb命令の実行
pub fn thumb_op_exec(_cpu: &mut CPU, _format: ThumbFormat, _op_format: ThumbInstruction) {
    match _format {
        ThumbFormat::Format01 => exec_op_format01(_cpu, _op_format),
        ThumbFormat::Format02 => exec_op_format02(_cpu, _op_format),
        ThumbFormat::Format03 => exec_op_format03(_cpu, _op_format),
        ThumbFormat::Format04 => exec_op_format04(_cpu, _op_format),
        ThumbFormat::Format05 => exec_op_format05(_cpu, _op_format),
        ThumbFormat::Format06 => exec_op_format06(_cpu, _op_format),
        ThumbFormat::Format07 => exec_op_format07(_cpu, _op_format),
        ThumbFormat::Format08 => exec_op_format08(_cpu, _op_format),
        ThumbFormat::Format09 => exec_op_format09(_cpu, _op_format),
        ThumbFormat::Format10 => exec_op_format10(_cpu, _op_format),
        ThumbFormat::Format11 => exec_op_format11(_cpu, _op_format),
        ThumbFormat::Format12 => exec_op_format12(_cpu, _op_format),
        ThumbFormat::Format13 => exec_op_format13(_cpu, _op_format),
        ThumbFormat::Format14 => exec_op_format14(_cpu, _op_format),
        ThumbFormat::Format15 => exec_op_format15(_cpu, _op_format),
        ThumbFormat::Format16 => exec_op_format16(_cpu, _op_format),
        ThumbFormat::Format17 => exec_op_format17(_cpu, _op_format),
        ThumbFormat::Format18 => exec_op_format18(_cpu, _op_format),
        ThumbFormat::Format19 => exec_op_format19(_cpu, _op_format),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thumb_format_decode() {
        // TODO Decode Test
    }
}