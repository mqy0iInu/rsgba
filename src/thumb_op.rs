#[allow(dead_code)]
pub fn thumb_op_decode(_op: u16) {
    let op = (_op & 0xFF00) >> 8; // 上位8ビットを取得
    let op_low = _op & 0x00FF; // 下位8ビットを取得

    match op {
        0x00 => match op_low {
            0x00 => println!("NOP"),
            0x01 => println!("MOV r0, r0"),
            0x02 => println!("MOV r0, r1"),
            0x03 => println!("MOV r0, r2"),
            0x04 => println!("MOV r0, r3"),
            0x05 => println!("MOV r0, r4"),
            // TODO 他の命令
            _ => panic!("Unknown Thumb Op!"),
        },
        0x01 => match op_low {
            0x00 => println!("LSL r0, r0"),
            0x01 => println!("LSL r0, r1"),
            0x02 => println!("LSL r0, r2"),
            0x03 => println!("LSL r0, r3"),
            0x04 => println!("LSL r0, r4"),
            // TODO 他の命令
            _ => panic!("Unknown Thumb Op!"),
        },
        0x02 => match op_low {
            0x00 => println!("LSR r0, r0"),
            0x01 => println!("LSR r0, r1"),
            0x02 => println!("LSR r0, r2"),
            0x03 => println!("LSR r0, r3"),
            0x04 => println!("LSR r0, r4"),
            // TODO 他の命令
            _ => panic!("Unknown Thumb Op!"),
        },
        // 他のオペコードのパターン
        _ => panic!("Unknown Thumb Op!"),
    }
}

#[allow(dead_code)]
pub fn thumb_op_exec(_op: u16) {
    
}