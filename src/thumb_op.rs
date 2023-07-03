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
    ConditionalBranch(u8, u8), // |Cond|Softset8|

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
    let softset8: u8  =  (instruction & 0b0000_0000_1111_1111) as u8;
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

pub fn thumb_format_decode(instruction: u16) -> ThumbInstruction {
    // Thumb命令のフォーマット判別（※ビットが立つ範囲で判定する）
    // FIXME Format1,2、Format16,17で最後のビット範囲が被る
    let format = match instruction {
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
        0b1101_0000_0000_0000..=0b1101_1111_1111_1111 => ThumbFormat::Format16,
        0b1101_1111_0000_0000..=0b1101_1111_1111_1111 => ThumbFormat::Format17,
        0b1110_0000_0000_0000..=0b1110_0111_1111_1111 => ThumbFormat::Format18,
        0b1111_0000_0000_0000..=0b1111_1111_1111_1111 => ThumbFormat::Format19,
        _ => panic!("Unknown Thumb instruction format"),
    };

    // Thumb命令のフォーマットに応じて命令をでコード
    match format {
        ThumbFormat::Format01 => decode_format01(instruction),
        ThumbFormat::Format02 => decode_format02(instruction),
        ThumbFormat::Format03 => decode_format03(instruction),
        ThumbFormat::Format04 => decode_format04(instruction),
        ThumbFormat::Format05 => decode_format05(instruction),
        ThumbFormat::Format06 => decode_format06(instruction),
        ThumbFormat::Format07 => decode_format07(instruction),
        ThumbFormat::Format08 => decode_format08(instruction),
        ThumbFormat::Format09 => decode_format09(instruction),
        ThumbFormat::Format10 => decode_format10(instruction),
        ThumbFormat::Format11 => decode_format11(instruction),
        ThumbFormat::Format12 => decode_format12(instruction),
        ThumbFormat::Format13 => decode_format13(instruction),
        ThumbFormat::Format14 => decode_format14(instruction),
        ThumbFormat::Format15 => decode_format15(instruction),
        ThumbFormat::Format16 => decode_format16(instruction),
        ThumbFormat::Format17 => decode_format17(instruction),
        ThumbFormat::Format18 => decode_format18(instruction),
        ThumbFormat::Format19 => decode_format19(instruction),
    }
}

// TODO ARM7TDMI Thumb命令のデコード
pub fn thumb_op_decode(_cpu: &mut CPU, _instruction: u16) -> ThumbInstruction{
    let _format_data = thumb_format_decode(_instruction);
    _format_data
}

// TODO ARM7TDMI Thumb命令の実行
pub fn thumb_op_exec(_cpu: &mut CPU, _format_data: ThumbInstruction) {
    todo!("ARM7TDMI Thumb命令の実行");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thumb_format_decode() {
        // TODO Decode Test
    }
}