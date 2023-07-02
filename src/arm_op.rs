#[allow(dead_code)]
pub fn arm_op_decode(_op: u32) {
    let op = (_op & 0xFF000000) >> 24; // 上位8ビットを取得
    let op_low = _op & 0x00FFFFFF; // 下位24ビットを取得

    match op {
        0x00 => match op_low {
            0x000000 => println!("NOP"),
            0x000001 => println!("MOV r0, r0"),
            0x000002 => println!("MOV r0, r1"),
            0x000003 => println!("MOV r0, r2"),
            0x000004 => println!("MOV r0, r3"),
            // 他の命令
            _ => panic!("Unknown ARM Op!"),
        },
        0x01 => match op_low {
            0x000000 => println!("ADD r0, r0, r0"),
            0x000001 => println!("ADD r0, r0, r1"),
            0x000002 => println!("ADD r0, r0, r2"),
            0x000003 => println!("ADD r0, r0, r3"),
            0x000004 => println!("ADD r0, r0, r4"),
            // 他の命令
            _ => panic!("Unknown ARM Op!"),
        },
        0x02 => match op_low {
            0x000000 => println!("SUB r0, r0, r0"),
            0x000001 => println!("SUB r0, r0, r1"),
            0x000002 => println!("SUB r0, r0, r2"),
            0x000003 => println!("SUB r0, r0, r3"),
            0x000004 => println!("SUB r0, r0, r4"),
            // 他の命令
            _ => panic!("Unknown ARM Op!"),
        },
        // 他のオペコードのパターン
        _ => panic!("Unknown ARM Op!"),
    }
}

#[allow(dead_code)]
pub fn arm_op_exec(_op: u32) {
    
}