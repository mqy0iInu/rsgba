use cpu::*;

// TODO Thumb命令のデコード
pub fn thumb_op_decode(_cpu: &mut CPU, _op: u16) {
    let op = (_op & 0xFF00) >> 8; // 上位8ビットを取得
    let op_low = _op & 0x00FF; // 下位8ビットを取得

    match op {
        0x00 => match op_low {
            0x00 => trace!("NOP"),
            0x01 => trace!("MOV r0, r0"),
            0x02 => trace!("MOV r0, r1"),
            0x03 => trace!("MOV r0, r2"),
            0x04 => trace!("MOV r0, r3"),
            0x05 => trace!("MOV r0, r4"),
            0x06 => trace!("MOV r0, r5"),
            0x07 => trace!("MOV r0, r6"),
            0x08 => trace!("MOV r0, r7"),
            0x09 => trace!("MOV r0, r8"),
            // TODO: Implement other instructions for op = 0x00
            _ => panic!("Unknown Thumb Op!"),
        },
        0x01 => match op_low {
            0x00 => trace!("LSL r0, r0"),
            0x01 => trace!("LSL r0, r1"),
            0x02 => trace!("LSL r0, r2"),
            0x03 => trace!("LSL r0, r3"),
            0x04 => trace!("LSL r0, r4"),
            0x05 => trace!("LSL r0, r5"),
            0x06 => trace!("LSL r0, r6"),
            0x07 => trace!("LSL r0, r7"),
            0x08 => trace!("LSL r0, r8"),
            // TODO: Implement other instructions for op = 0x01
            _ => panic!("Unknown Thumb Op!"),
        },
        0x02 => match op_low {
            0x00 => trace!("LSR r0, r0"),
            0x01 => trace!("LSR r0, r1"),
            0x02 => trace!("LSR r0, r2"),
            0x03 => trace!("LSR r0, r3"),
            0x04 => trace!("LSR r0, r4"),
            0x05 => trace!("LSR r0, r5"),
            0x06 => trace!("LSR r0, r6"),
            0x07 => trace!("LSR r0, r7"),
            0x08 => trace!("LSR r0, r8"),
            // TODO: Implement other instructions for op = 0x02
            _ => panic!("Unknown Thumb Op!"),
        },
        0x03 => match op_low {
            0x00 => trace!("ADD r0, r0"),
            0x01 => trace!("ADD r0, r1"),
            0x02 => trace!("ADD r0, r2"),
            0x03 => trace!("ADD r0, r3"),
            0x04 => trace!("ADD r0, r4"),
            0x05 => trace!("ADD r0, r5"),
            0x06 => trace!("ADD r0, r6"),
            0x07 => trace!("ADD r0, r7"),
            0x08 => trace!("ADD r0, r8"),
            // TODO: Implement other instructions for op = 0x03
            _ => panic!("Unknown Thumb Op!"),
        },
        0x04 => match op_low {
            0x00 => trace!("SUB r0, r0"),
            0x01 => trace!("SUB r0, r1"),
            0x02 => trace!("SUB r0, r2"),
            0x03 => trace!("SUB r0, r3"),
            0x04 => trace!("SUB r0, r4"),
            0x05 => trace!("SUB r0, r5"),
            0x06 => trace!("SUB r0, r6"),
            0x07 => trace!("SUB r0, r7"),
            0x08 => trace!("SUB r0, r8"),
            // TODO: Implement other instructions for op = 0x04
            _ => panic!("Unknown Thumb Op!"),
        },
        0x05 => match op_low {
            0x00 => trace!("CMP r0, r0"),
            0x01 => trace!("CMP r0, r1"),
            0x02 => trace!("CMP r0, r2"),
            0x03 => trace!("CMP r0, r3"),
            0x04 => trace!("CMP r0, r4"),
            0x05 => trace!("CMP r0, r5"),
            0x06 => trace!("CMP r0, r6"),
            0x07 => trace!("CMP r0, r7"),
            0x08 => trace!("CMP r0, r8"),
            // TODO: Implement other instructions for op = 0x05
            _ => panic!("Unknown Thumb Op!"),
        },
        // 追加のオペコード
        0x06 => match op_low {
            0x00 => trace!("AND r0, r0"),
            0x01 => trace!("AND r0, r1"),
            0x02 => trace!("AND r0, r2"),
            // ... 他のAND命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0x07 => match op_low {
            0x00 => trace!("ORR r0, r0"),
            0x01 => trace!("ORR r0, r1"),
            0x02 => trace!("ORR r0, r2"),
            // ... 他のORR命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0x08 => match op_low {
            0x00 => trace!("EOR r0, r0"),
            0x01 => trace!("EOR r0, r1"),
            0x02 => trace!("EOR r0, r2"),
            // ... 他のEOR命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0x09 => match op_low {
            0x00 => trace!("MOV r0, r3"),
            0x01 => trace!("MOV r0, r4"),
            0x02 => trace!("MOV r0, r5"),
            // ... 他のMOV命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA0 => match op_low {
            0x00 => trace!("STR r0, [r0]"),
            0x01 => trace!("STR r0, [r1]"),
            0x02 => trace!("STR r0, [r2]"),
            // ... 他のSTR命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA1 => match op_low {
            0x00 => trace!("LDR r0, [r0]"),
            0x01 => trace!("LDR r0, [r1]"),
            0x02 => trace!("LDR r0, [r2]"),
            // ... 他のLDR命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA2 => match op_low {
            0x00 => trace!("STRB r0, [r0]"),
            0x01 => trace!("STRB r0, [r1]"),
            0x02 => trace!("STRB r0, [r2]"),
            // ... 他のSTRB命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA3 => match op_low {
            0x00 => trace!("LDRB r0, [r0]"),
            0x01 => trace!("LDRB r0, [r1]"),
            0x02 => trace!("LDRB r0, [r2]"),
            // ... 他のLDRB命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA4 => match op_low {
            0x00 => trace!("STRH r0, [r0]"),
            0x01 => trace!("STRH r0, [r1]"),
            0x02 => trace!("STRH r0, [r2]"),
            // ... 他のSTRH命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA5 => match op_low {
            0x00 => trace!("LDRH r0, [r0]"),
            0x01 => trace!("LDRH r0, [r1]"),
            0x02 => trace!("LDRH r0, [r2]"),
            // ... 他のLDRH命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA6 => match op_low {
            0x00 => trace!("STRH r0, [r0, r1]"),
            0x01 => trace!("STRH r0, [r1, r2]"),
            0x02 => trace!("STRH r0, [r2, r3]"),
            // ... 他のSTRH命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA7 => match op_low {
            0x00 => trace!("LDRH r0, [r0, r1]"),
            0x01 => trace!("LDRH r0, [r1, r2]"),
            0x02 => trace!("LDRH r0, [r2, r3]"),
            // ... 他のLDRH命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },

        0xA8 => match op_low {
            0x00 => trace!("ADD r0, PC"),
            0x01 => trace!("ADD r1, PC"),
            0x02 => trace!("ADD r2, PC"),
            // ... 他のADD PC命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xA9 => match op_low {
            0x00 => trace!("ADD r0, SP"),
            0x01 => trace!("ADD r1, SP"),
            0x02 => trace!("ADD r2, SP"),
            // ... 他のADD SP命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xAA => match op_low {
            0x00 => trace!("ADD SP, #0"),
            0x01 => trace!("ADD SP, #1"),
            0x02 => trace!("ADD SP, #2"),
            // ... 他のADD SP, #imm命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xAB => match op_low {
            0x00 => trace!("SUB SP, #0"),
            0x01 => trace!("SUB SP, #1"),
            0x02 => trace!("SUB SP, #2"),
            // ... 他のSUB SP, #imm命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xAC => match op_low {
            0x00 => trace!("ADD r0, r0, r0"),
            0x01 => trace!("ADD r0, r0, r1"),
            0x02 => trace!("ADD r0, r0, r2"),
            0x03 => trace!("ADD r0, r0, r3"),
            0x04 => trace!("ADD r0, r0, r4"),
            0x05 => trace!("ADD r0, r0, r5"),
            0x06 => trace!("ADD r0, r0, r6"),
            0x07 => trace!("ADD r0, r0, r7"),
            0x08 => trace!("ADD r0, r0, r8"),
            0x09 => trace!("ADD r0, r0, r9"),
            // ... 他のADD命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xAD => match op_low {
            0x00 => trace!("SUB r0, r0, r0"),
            0x01 => trace!("SUB r0, r0, r1"),
            0x02 => trace!("SUB r0, r0, r2"),
            0x03 => trace!("SUB r0, r0, r3"),
            0x04 => trace!("SUB r0, r0, r4"),
            0x05 => trace!("SUB r0, r0, r5"),
            0x06 => trace!("SUB r0, r0, r6"),
            0x07 => trace!("SUB r0, r0, r7"),
            0x08 => trace!("SUB r0, r0, r8"),
            0x09 => trace!("SUB r0, r0, r9"),
            // ... 他のSUB命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xAE => match op_low {
            0x00 => trace!("ADD r0, r0, #0"),
            0x01 => trace!("ADD r0, r0, #1"),
            0x02 => trace!("ADD r0, r0, #2"),
            0x03 => trace!("ADD r0, r0, #3"),
            0x04 => trace!("ADD r0, r0, #4"),
            0x05 => trace!("ADD r0, r0, #5"),
            0x06 => trace!("ADD r0, r0, #6"),
            0x07 => trace!("ADD r0, r0, #7"),
            0x08 => trace!("ADD r0, r0, #8"),
            0x09 => trace!("ADD r0, r0, #9"),
            // ... 他のADD #imm命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xAF => match op_low {
            0x00 => trace!("SUB r0, r0, #0"),
            0x01 => trace!("SUB r0, r0, #1"),
            0x02 => trace!("SUB r0, r0, #2"),
            0x03 => trace!("SUB r0, r0, #3"),
            0x04 => trace!("SUB r0, r0, #4"),
            0x05 => trace!("SUB r0, r0, #5"),
            0x06 => trace!("SUB r0, r0, #6"),
            0x07 => trace!("SUB r0, r0, #7"),
            0x08 => trace!("SUB r0, r0, #8"),
            0x09 => trace!("SUB r0, r0, #9"),
            // ... 他のSUB #imm命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xB0 => match op_low {
            0x00 => trace!("MOV r0, r0, LSL #0"),
            0x01 => trace!("MOV r0, r0, LSL #1"),
            0x02 => trace!("MOV r0, r0, LSL #2"),
            0x03 => trace!("MOV r0, r0, LSL #3"),
            0x04 => trace!("MOV r0, r0, LSL #4"),
            0x05 => trace!("MOV r0, r0, LSL #5"),
            0x06 => trace!("MOV r0, r0, LSL #6"),
            0x07 => trace!("MOV r0, r0, LSL #7"),
            0x08 => trace!("MOV r0, r0, LSL #8"),
            0x09 => trace!("MOV r0, r0, LSL #9"),
            // ... 他のLSL命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xB1 => match op_low {
            0x00 => trace!("MOV r0, r0, LSR #0"),
            0x01 => trace!("MOV r0, r0, LSR #1"),
            0x02 => trace!("MOV r0, r0, LSR #2"),
            0x03 => trace!("MOV r0, r0, LSR #3"),
            0x04 => trace!("MOV r0, r0, LSR #4"),
            0x05 => trace!("MOV r0, r0, LSR #5"),
            0x06 => trace!("MOV r0, r0, LSR #6"),
            0x07 => trace!("MOV r0, r0, LSR #7"),
            0x08 => trace!("MOV r0, r0, LSR #8"),
            0x09 => trace!("MOV r0, r0, LSR #9"),
            // ... 他のLSR命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xB2 => match op_low {
            0x00 => trace!("MOV r0, r0, ASR #0"),
            0x01 => trace!("MOV r0, r0, ASR #1"),
            0x02 => trace!("MOV r0, r0, ASR #2"),
            0x03 => trace!("MOV r0, r0, ASR #3"),
            0x04 => trace!("MOV r0, r0, ASR #4"),
            0x05 => trace!("MOV r0, r0, ASR #5"),
            0x06 => trace!("MOV r0, r0, ASR #6"),
            0x07 => trace!("MOV r0, r0, ASR #7"),
            0x08 => trace!("MOV r0, r0, ASR #8"),
            0x09 => trace!("MOV r0, r0, ASR #9"),
            // ... 他のASR命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xB3 => match op_low {
            0x00 => trace!("ADD r0, r0, #0x00"),
            0x01 => trace!("ADD r0, r0, #0x01"),
            0x02 => trace!("ADD r0, r0, #0x02"),
            0x03 => trace!("ADD r0, r0, #0x03"),
            0x04 => trace!("ADD r0, r0, #0x04"),
            0x05 => trace!("ADD r0, r0, #0x05"),
            0x06 => trace!("ADD r0, r0, #0x06"),
            0x07 => trace!("ADD r0, r0, #0x07"),
            0x08 => trace!("ADD r0, r0, #0x08"),
            0x09 => trace!("ADD r0, r0, #0x09"),
            // ... 他のADD命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xB4 => match op_low {
            0x00 => trace!("SUB r0, r0, #0x00"),
            0x01 => trace!("SUB r0, r0, #0x01"),
            0x02 => trace!("SUB r0, r0, #0x02"),
            0x03 => trace!("SUB r0, r0, #0x03"),
            0x04 => trace!("SUB r0, r0, #0x04"),
            0x05 => trace!("SUB r0, r0, #0x05"),
            0x06 => trace!("SUB r0, r0, #0x06"),
            0x07 => trace!("SUB r0, r0, #0x07"),
            0x08 => trace!("SUB r0, r0, #0x08"),
            0x09 => trace!("SUB r0, r0, #0x09"),
            // ... 他のSUB命令の実装
            _ => panic!("Unknown Thumb Op!"),
        },
        0xB5 => match op_low {
            0x00 => trace!("ADD r0, r0, #0x00"),
            0x01 => trace!("ADD r0, r0, #0x01"),
            0x02 => trace!("ADD r0, r0, #0x02"),
            0x03 => trace!("ADD r0, r0, #0x03"),
            0x04 => trace!("ADD r0, r0, #0x04"),
            0x05 => trace!("ADD r0, r0, #0x05"),
            0x06 => trace!("ADD r0, r0, #0x06"),
            0x07 => trace!("ADD r0, r0, #0x07"),
            0x08 => trace!("ADD r0, r0, #0x08"),
            0x09 => trace!("ADD r0, r0, #0x09"),
            0x0A => trace!("ADD r0, r0, #0x0A"),
            0x0B => trace!("ADD r0, r0, #0x0B"),
            0x0C => trace!("ADD r0, r0, #0x0C"),
            0x0D => trace!("ADD r0, r0, #0x0D"),
            0x0E => trace!("ADD r0, r0, #0x0E"),
            0x0F => trace!("ADD r0, r0, #0x0F"),
            0x10 => trace!("ADD r0, r0, #0x10"),
            0x11 => trace!("ADD r0, r0, #0x11"),
            0x12 => trace!("ADD r0, r0, #0x12"),
            0x13 => trace!("ADD r0, r0, #0x13"),
            // ... 他のADD命令の実装
            _ => panic!("Unknown Thumb Op!"),
            },
            0xB6 => match op_low {
                0x00 => trace!("SUB r0, r0, #0x00"),
                0x01 => trace!("SUB r0, r0, #0x01"),
                0x02 => trace!("SUB r0, r0, #0x02"),
                0x03 => trace!("SUB r0, r0, #0x03"),
                0x04 => trace!("SUB r0, r0, #0x04"),
                0x05 => trace!("SUB r0, r0, #0x05"),
                0x06 => trace!("SUB r0, r0, #0x06"),
                0x07 => trace!("SUB r0, r0, #0x07"),
                0x08 => trace!("SUB r0, r0, #0x08"),
                0x09 => trace!("SUB r0, r0, #0x09"),
                0x0A => trace!("SUB r0, r0, #0x0A"),
                0x0B => trace!("SUB r0, r0, #0x0B"),
                0x0C => trace!("SUB r0, r0, #0x0C"),
                0x0D => trace!("SUB r0, r0, #0x0D"),
                0x0E => trace!("SUB r0, r0, #0x0E"),
                0x0F => trace!("SUB r0, r0, #0x0F"),
                0x10 => trace!("SUB r0, r0, #0x10"),
                0x11 => trace!("SUB r0, r0, #0x11"),
                0x12 => trace!("SUB r0, r0, #0x12"),
                0x13 => trace!("SUB r0, r0, #0x13"),
                // ... 他のSUB命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xB7 => match op_low {
                0x00 => trace!("CMP r0, #0x00"),
                0x01 => trace!("CMP r0, #0x01"),
                0x02 => trace!("CMP r0, #0x02"),
                0x03 => trace!("CMP r0, #0x03"),
                0x04 => trace!("CMP r0, #0x04"),
                0x05 => trace!("CMP r0, #0x05"),
                0x06 => trace!("CMP r0, #0x06"),
                0x07 => trace!("CMP r0, #0x07"),
                0x08 => trace!("CMP r0, #0x08"),
                0x09 => trace!("CMP r0, #0x09"),
                0x0A => trace!("CMP r0, #0x0A"),
                0x0B => trace!("CMP r0, #0x0B"),
                0x0C => trace!("CMP r0, #0x0C"),
                0x0D => trace!("CMP r0, #0x0D"),
                0x0E => trace!("CMP r0, #0x0E"),
                0x0F => trace!("CMP r0, #0x0F"),
                0x10 => trace!("CMP r0, #0x10"),
                0x11 => trace!("CMP r0, #0x11"),
                0x12 => trace!("CMP r0, #0x12"),
                0x13 => trace!("CMP r0, #0x13"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xB8 => match op_low {
                0x00 => trace!("MOV r0, r1, LSL #0x00"),
                0x01 => trace!("MOV r0, r1, LSL #0x01"),
                0x02 => trace!("MOV r0, r1, LSL #0x02"),
                0x03 => trace!("MOV r0, r1, LSL #0x03"),
                0x04 => trace!("MOV r0, r1, LSL #0x04"),
                0x05 => trace!("MOV r0, r1, LSL #0x05"),
                0x06 => trace!("MOV r0, r1, LSL #0x06"),
                0x07 => trace!("MOV r0, r1, LSL #0x07"),
                0x08 => trace!("MOV r0, r1, LSL #0x08"),
                0x09 => trace!("MOV r0, r1, LSL #0x09"),
                0x0A => trace!("MOV r0, r1, LSL #0x0A"),
                0x0B => trace!("MOV r0, r1, LSL #0x0B"),
                0x0C => trace!("MOV r0, r1, LSL #0x0C"),
                0x0D => trace!("MOV r0, r1, LSL #0x0D"),
                0x0E => trace!("MOV r0, r1, LSL #0x0E"),
                0x0F => trace!("MOV r0, r1, LSL #0x0F"),
                0x10 => trace!("MOV r0, r1, LSL #0x10"),
                0x11 => trace!("MOV r0, r1, LSL #0x11"),
                0x12 => trace!("MOV r0, r1, LSL #0x12"),
                0x13 => trace!("MOV r0, r1, LSL #0x13"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xB9 => match op_low {
                0x00 => trace!("MOV r0, r1, LSR #0x00"),
                0x01 => trace!("MOV r0, r1, LSR #0x01"),
                0x02 => trace!("MOV r0, r1, LSR #0x02"),
                0x03 => trace!("MOV r0, r1, LSR #0x03"),
                0x04 => trace!("MOV r0, r1, LSR #0x04"),
                0x05 => trace!("MOV r0, r1, LSR #0x05"),
                0x06 => trace!("MOV r0, r1, LSR #0x06"),
                0x07 => trace!("MOV r0, r1, LSR #0x07"),
                0x08 => trace!("MOV r0, r1, LSR #0x08"),
                0x09 => trace!("MOV r0, r1, LSR #0x09"),
                0x0A => trace!("MOV r0, r1, LSR #0x0A"),
                0x0B => trace!("MOV r0, r1, LSR #0x0B"),
                0x0C => trace!("MOV r0, r1, LSR #0x0C"),
                0x0D => trace!("MOV r0, r1, LSR #0x0D"),
                0x0E => trace!("MOV r0, r1, LSR #0x0E"),
                0x0F => trace!("MOV r0, r1, LSR #0x0F"),
                0x10 => trace!("MOV r0, r1, LSR #0x10"),
                0x11 => trace!("MOV r0, r1, LSR #0x11"),
                0x12 => trace!("MOV r0, r1, LSR #0x12"),
                0x13 => trace!("MOV r0, r1, LSR #0x13"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xBA => match op_low {
                0x00 => trace!("MOV r0, r1, ASR #0x00"),
                0x01 => trace!("MOV r0, r1, ASR #0x01"),
                0x02 => trace!("MOV r0, r1, ASR #0x02"),
                0x03 => trace!("MOV r0, r1, ASR #0x03"),
                0x04 => trace!("MOV r0, r1, ASR #0x04"),
                0x05 => trace!("MOV r0, r1, ASR #0x05"),
                0x06 => trace!("MOV r0, r1, ASR #0x06"),
                0x07 => trace!("MOV r0, r1, ASR #0x07"),
                0x08 => trace!("MOV r0, r1, ASR #0x08"),
                0x09 => trace!("MOV r0, r1, ASR #0x09"),
                0x0A => trace!("MOV r0, r1, ASR #0x0A"),
                0x0B => trace!("MOV r0, r1, ASR #0x0B"),
                0x0C => trace!("MOV r0, r1, ASR #0x0C"),
                0x0D => trace!("MOV r0, r1, ASR #0x0D"),
                0x0E => trace!("MOV r0, r1, ASR #0x0E"),
                0x0F => trace!("MOV r0, r1, ASR #0x0F"),
                0x10 => trace!("MOV r0, r1, ASR #0x10"),
                0x11 => trace!("MOV r0, r1, ASR #0x11"),
                0x12 => trace!("MOV r0, r1, ASR #0x12"),
                0x13 => trace!("MOV r0, r1, ASR #0x13"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xBB => match op_low {
                0x00 => trace!("MOV r0, r1, ROR #0x00"),
                0x01 => trace!("MOV r0, r1, ROR #0x01"),
                0x02 => trace!("MOV r0, r1, ROR #0x02"),
                0x03 => trace!("MOV r0, r1, ROR #0x03"),
                0x04 => trace!("MOV r0, r1, ROR #0x04"),
                0x05 => trace!("MOV r0, r1, ROR #0x05"),
                0x06 => trace!("MOV r0, r1, ROR #0x06"),
                0x07 => trace!("MOV r0, r1, ROR #0x07"),
                0x08 => trace!("MOV r0, r1, ROR #0x08"),
                0x09 => trace!("MOV r0, r1, ROR #0x09"),
                0x0A => trace!("MOV r0, r1, ROR #0x0A"),
                0x0B => trace!("MOV r0, r1, ROR #0x0B"),
                0x0C => trace!("MOV r0, r1, ROR #0x0C"),
                0x0D => trace!("MOV r0, r1, ROR #0x0D"),
                0x0E => trace!("MOV r0, r1, ROR #0x0E"),
                0x0F => trace!("MOV r0, r1, ROR #0x0F"),
                0x10 => trace!("MOV r0, r1, ROR #0x10"),
                0x11 => trace!("MOV r0, r1, ROR #0x11"),
                0x12 => trace!("MOV r0, r1, ROR #0x12"),
                0x13 => trace!("MOV r0, r1, ROR #0x13"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xBC => match op_low {
                0x00 => trace!("NOP"),
                0x01 => trace!("YIELD"),
                0x02 => trace!("WFE"),
                0x03 => trace!("WFI"),
                0x04 => trace!("SEV"),
                0x05 => trace!("SEVL"),
                // ... 他のシステム制御命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xBD => match op_low {
                0x00 => trace!("DBG"),
                // ... 他のデバッグ命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xBE => match op_low {
                0x00 => trace!("BKPT #0x00"),
                // ... 他のBKPT命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xBF => match op_low {
                0x00 => trace!("DPLI"),
                0x01 => trace!("DRBIT"),
                0x02 => trace!("DRUN"),
                // ... 他のデバッグ命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC0 => match op_low {
                0x00 => trace!("SETEND BE"),
                0x01 => trace!("SETEND LE"),
                // ... 他のSETEND命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC1 => match op_low {
                0x00 => trace!("CP15"),
                // ... 他のCP15命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC2 => match op_low {
                0x00 => trace!("CMP r0, #0x00"),
                0x01 => trace!("CMP r0, #0x01"),
                0x02 => trace!("CMP r0, #0x02"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC3 => match op_low {
                0x00 => trace!("CMN r0, #0x00"),
                0x01 => trace!("CMN r0, #0x01"),
                0x02 => trace!("CMN r0, #0x02"),
                // ... 他のCMN命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC4 => match op_low {
                0x00 => trace!("ORR r0, r0"),
                0x01 => trace!("ORR r0, r1"),
                0x02 => trace!("ORR r0, r2"),
                // ... 他のORR命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC5 => match op_low {
                0x00 => trace!("MUL r0, r0"),
                0x01 => trace!("MUL r0, r1"),
                0x02 => trace!("MUL r0, r2"),
                // ... 他のMUL命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC6 => match op_low {
                0x00 => trace!("BIC r0, r0"),
                0x01 => trace!("BIC r0, r1"),
                0x02 => trace!("BIC r0, r2"),
                // ... 他のBIC命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC7 => match op_low {
                0x00 => trace!("MVN r0, r0"),
                0x01 => trace!("MVN r0, r1"),
                0x02 => trace!("MVN r0, r2"),
                // ... 他のMVN命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC8 => match op_low {
                0x00 => trace!("ADD r0, #0x00"),
                0x01 => trace!("ADD r0, #0x01"),
                0x02 => trace!("ADD r0, #0x02"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xC9 => match op_low {
                0x00 => trace!("CMP r0, #0x00"),
                0x01 => trace!("CMP r0, #0x01"),
                0x02 => trace!("CMP r0, #0x02"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xCA => match op_low {
                0x00 => trace!("MOV r0, #0x00"),
                0x01 => trace!("MOV r0, #0x01"),
                0x02 => trace!("MOV r0, #0x02"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xCB => match op_low {
                0x00 => trace!("ADD r0, r0, #0x00"),
                0x01 => trace!("ADD r0, r0, #0x01"),
                0x02 => trace!("ADD r0, r0, #0x02"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xCC => match op_low {
                0x00 => trace!("CMP r0, r0, #0x00"),
                0x01 => trace!("CMP r0, r0, #0x01"),
                0x02 => trace!("CMP r0, r0, #0x02"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xCD => match op_low {
                0x00 => trace!("MOV r0, r0, LSL #0x00"),
                0x01 => trace!("MOV r0, r0, LSL #0x01"),
                0x02 => trace!("MOV r0, r0, LSL #0x02"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xCE => match op_low {
                0x00 => trace!("MOV r0, r0, LSR #0x00"),
                0x01 => trace!("MOV r0, r0, LSR #0x01"),
                0x02 => trace!("MOV r0, r0, LSR #0x02"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xCF => match op_low {
                0x00 => trace!("MOV r0, r0, ASR #0x00"),
                0x01 => trace!("MOV r0, r0, ASR #0x01"),
                0x02 => trace!("MOV r0, r0, ASR #0x02"),
                // ... 他のMOV命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD0 => match op_low {
                0x00 => trace!("BEQ offset"),
                0x01 => trace!("BNE offset"),
                0x02 => trace!("BCS offset"),
                0x03 => trace!("BCC offset"),
                0x04 => trace!("BMI offset"),
                0x05 => trace!("BPL offset"),
                0x06 => trace!("BVS offset"),
                0x07 => trace!("BVC offset"),
                0x08 => trace!("BHI offset"),
                0x09 => trace!("BLS offset"),
                0x0A => trace!("BGE offset"),
                0x0B => trace!("BLT offset"),
                0x0C => trace!("BGT offset"),
                0x0D => trace!("BLE offset"),
                // ... 他のBコンディショナル命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD1 => match op_low {
                0x00 => trace!("LDRB r0, [r0, r0]"),
                0x01 => trace!("LDRB r0, [r0, r1]"),
                0x02 => trace!("LDRB r0, [r0, r2]"),
                // ... 他のLDRB命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD2 => match op_low {
                0x00 => trace!("LDR r0, [r0, r0]"),
                0x01 => trace!("LDR r0, [r0, r1]"),
                0x02 => trace!("LDR r0, [r0, r2]"),
                // ... 他のLDR命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD3 => match op_low {
                0x00 => trace!("STRB r0, [r0, r0]"),
                0x01 => trace!("STRB r0, [r0, r1]"),
                0x02 => trace!("STRB r0, [r0, r2]"),
                // ... 他のSTRB命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD4 => match op_low {
                0x00 => trace!("STR r0, [r0, r0]"),
                0x01 => trace!("STR r0, [r0, r1]"),
                0x02 => trace!("STR r0, [r0, r2]"),
                // ... 他のSTR命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD5 => match op_low {
                0x00 => trace!("LDRH r0, [r0, r0]"),
                0x01 => trace!("LDRH r0, [r0, r1]"),
                0x02 => trace!("LDRH r0, [r0, r2]"),
                // ... 他のLDRH命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD6 => match op_low {
                0x00 => trace!("LDR r0, [SP, #offset]"),
                // ... 他のLDR命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD7 => match op_low {
                0x00 => trace!("STRB r0, [SP, #offset]"),
                // ... 他のSTRB命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD8 => match op_low {
                0x00 => trace!("LDRH r0, [SP, #offset]"),
                // ... 他のLDRH命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xD9 => match op_low {
                0x00 => trace!("LDR r0, [PC, #offset]"),
                // ... 他のLDR命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xDA => match op_low {
                0x00 => trace!("LDRB r0, [PC, #offset]"),
                // ... 他のLDRB命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xDB => match op_low {
                0x00 => trace!("LDRH r0, [PC, #offset]"),
                // ... 他のLDRH命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xDC => match op_low {
                0x00 => trace!("ADD r0, PC, #offset"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xDD => match op_low {
                0x00 => trace!("ADD r0, SP, #offset"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xDE => match op_low {
                0x00 => trace!("ADD SP, #offset"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xDF => match op_low {
                0x00 => trace!("POP {{r0}}"),
                0x01 => trace!("POP {{r0, r1}}"),
                0x02 => trace!("POP {{r0, r1, r2}}"),
                // ... 他のPOP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE0 => match op_low {
                0x00 => trace!("SUB SP, #offset"),
                // ... 他のSUB命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE1 => match op_low {
                0x00 => trace!("BIC r0, r0"),
                0x01 => trace!("BIC r0, r1"),
                0x02 => trace!("BIC r0, r2"),
                // ... 他のBIC命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE2 => match op_low {
                0x00 => trace!("BIC r0, r0, LSL #0x00"),
                0x01 => trace!("BIC r0, r0, LSL #0x01"),
                0x02 => trace!("BIC r0, r0, LSL #0x02"),
                // ... 他のBIC命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE3 => match op_low {
                0x00 => trace!("BIC r0, r0, LSR #0x00"),
                0x01 => trace!("BIC r0, r0, LSR #0x01"),
                0x02 => trace!("BIC r0, r0, LSR #0x02"),
                // ... 他のBIC命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE4 => match op_low {
                0x00 => trace!("BIC r0, r0, ASR #0x00"),
                0x01 => trace!("BIC r0, r0, ASR #0x01"),
                0x02 => trace!("BIC r0, r0, ASR #0x02"),
                // ... 他のBIC命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE5 => match op_low {
                0x00 => trace!("BIC r0, r0, ROR #0x00"),
                0x01 => trace!("BIC r0, r0, ROR #0x01"),
                0x02 => trace!("BIC r0, r0, ROR #0x02"),
                // ... 他のBIC命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE6 => match op_low {
                0x00 => trace!("MVN r0, r0"),
                0x01 => trace!("MVN r0, r1"),
                0x02 => trace!("MVN r0, r2"),
                // ... 他のMVN命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE7 => match op_low {
                0x00 => trace!("MVN r0, r0, LSL #0x00"),
                0x01 => trace!("MVN r0, r0, LSL #0x01"),
                0x02 => trace!("MVN r0, r0, LSL #0x02"),
                // ... 他のMVN命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE8 => match op_low {
                0x00 => trace!("MVN r0, r0, LSR #0x00"),
                0x01 => trace!("MVN r0, r0, LSR #0x01"),
                0x02 => trace!("MVN r0, r0, LSR #0x02"),
                // ... 他のMVN命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xE9 => match op_low {
                0x00 => trace!("MVN r0, r0, ASR #0x00"),
                0x01 => trace!("MVN r0, r0, ASR #0x01"),
                0x02 => trace!("MVN r0, r0, ASR #0x02"),
                // ... 他のMVN命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xEA => match op_low {
                0x00 => trace!("MVN r0, r0, ROR #0x00"),
                0x01 => trace!("MVN r0, r0, ROR #0x01"),
                0x02 => trace!("MVN r0, r0, ROR #0x02"),
                // ... 他のMVN命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xEB => match op_low {
                0x00 => trace!("BLX r0"),
                // ... 他のBLX命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xEC => match op_low {
                0x00 => trace!("BX r0"),
                // ... 他のBX命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xED => match op_low {
                0x00 => trace!("BL prefix instruction"),
                // ... 他のBL prefix命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xEE => match op_low {
                0x00 => trace!("BL suffix instruction"),
                // ... 他のBL suffix命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xEF => match op_low {
                0x00 => trace!("BL instruction"),
                // ... 他のBL命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF0 => match op_low {
                0x00 => trace!("ADD r0, #immediate"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF1 => match op_low {
                0x00 => trace!("ADD r0, r0, #immediate"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF2 => match op_low {
                0x00 => trace!("ADD r0, r0, r0"),
                0x01 => trace!("ADD r0, r0, r1"),
                0x02 => trace!("ADD r0, r0, r2"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF3 => match op_low {
                0x00 => trace!("ADD r0, r0, r0, LSL #0x00"),
                0x01 => trace!("ADD r0, r0, r0, LSL #0x01"),
                0x02 => trace!("ADD r0, r0, r0, LSL #0x02"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF4 => match op_low {
                0x00 => trace!("ADD r0, r0, r0, LSR #0x00"),
                0x01 => trace!("ADD r0, r0, r0, LSR #0x01"),
                0x02 => trace!("ADD r0, r0, r0, LSR #0x02"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF5 => match op_low {
                0x00 => trace!("ADD r0, r0, r0, ASR #0x00"),
                0x01 => trace!("ADD r0, r0, r0, ASR #0x01"),
                0x02 => trace!("ADD r0, r0, r0, ASR #0x02"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF6 => match op_low {
                0x00 => trace!("ADD r0, r0, r0, ROR #0x00"),
                0x01 => trace!("ADD r0, r0, r0, ROR #0x01"),
                0x02 => trace!("ADD r0, r0, r0, ROR #0x02"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF7 => match op_low {
                0x00 => trace!("ADD sp, #immediate"),
                // ... 他のADD命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF8 => match op_low {
                0x00 => trace!("CMP r0, #immediate"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xF9 => match op_low {
                0x00 => trace!("CMP r0, r0"),
                0x01 => trace!("CMP r0, r1"),
                0x02 => trace!("CMP r0, r2"),
                 // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xFA => match op_low {
                0x00 => trace!("CMP r0, r0, LSL #0x00"),
                0x01 => trace!("CMP r0, r0, LSL #0x01"),
                0x02 => trace!("CMP r0, r0, LSL #0x02"),
                 // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xFB => match op_low {
                0x00 => trace!("CMP r0, r0, LSR #0x00"),
                0x01 => trace!("CMP r0, r0, LSR #0x01"),
                0x02 => trace!("CMP r0, r0, LSR #0x02"),
                 // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xFC => match op_low {
                0x00 => trace!("CMP r0, r0, ASR #0x00"),
                0x01 => trace!("CMP r0, r0, ASR #0x01"),
                0x02 => trace!("CMP r0, r0, ASR #0x02"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xFD => match op_low {
                0x00 => trace!("CMP r0, r0, ROR #0x00"),
                0x01 => trace!("CMP r0, r0, ROR #0x01"),
                0x02 => trace!("CMP r0, r0, ROR #0x02"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xFE => match op_low {
                0x00 => trace!("CMP sp, #immediate"),
                // ... 他のCMP命令の実装
                _ => panic!("Unknown Thumb Op!"),
            },
            0xFF => match op_low {
                0x00 => trace!("SWI #immediate"),
                0x01 => trace!("BLX r1"),
                0x02 => trace!("BX r2"),
                0x03 => trace!("BLX r3"),
                0x04 => trace!("BLX r4"),
                0x05 => trace!("BLX r5"),
                0x06 => trace!("BLX r6"),
                0x07 => trace!("BLX r7"),
                0x08 => trace!("BLX r8"),
                0x09 => trace!("BLX r9"),
                0x0A => trace!("BLX r10"),
                0x0B => trace!("BLX r11"),
                0x0C => trace!("BLX r12"),
                0x0D => trace!("BLX r13"),
                0x0E => trace!("BLX r14"),
                0x0F => trace!("BLX r15"),
                _ => panic!("Unknown Thumb Op!"),
            },
        // TODO: Implement other opcodes
        _ => panic!("Unknown Thumb Op!"),
    }
}

// TODO Thumb命令の実行
pub fn thumb_op_exec(_cpu: &mut CPU, _op: u16) {
    todo!("Plz! ARM Thumb OP Exec Func Impl!")
}