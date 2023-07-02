use cpu::*;

// TODO ARM命令のデコード
#[allow(dead_code)]
pub fn arm_op_decode(_cpu: &mut CPU, _op: u32) {
    let op = (_op & 0xFF000000) >> 24; // 上位8ビットを取得
    let op_low = _op & 0x00FFFFFF; // 下位24ビットを取得

    match op {
        0x00 => match op_low {
            0x000000 => trace!("NOP"),
            0x000001 => trace!("MOV r0, r0"),
            0x000002 => trace!("MOV r0, r1"),
            0x000003 => trace!("MOV r0, r2"),
            0x000004 => trace!("MOV r0, r3"),
            // 他の命令
            _ => panic!("Unknown ARM Op!"),
        },
        0x01 => match op_low {
            0x000000 => trace!("ADD r0, r0, r0"),
            0x000001 => trace!("ADD r0, r0, r1"),
            0x000002 => trace!("ADD r0, r0, r2"),
            0x000003 => trace!("ADD r0, r0, r3"),
            0x000004 => trace!("ADD r0, r0, r4"),
            // 他の命令
            _ => panic!("Unknown ARM Op!"),
        },
        0x02 => match op_low {
            0x000000 => trace!("SUB r0, r0, r0"),
            0x000001 => trace!("SUB r0, r0, r1"),
            0x000002 => trace!("SUB r0, r0, r2"),
            0x000003 => trace!("SUB r0, r0, r3"),
            0x000004 => trace!("SUB r0, r0, r4"),
            // 他の命令
            _ => panic!("Unknown ARM Op!"),
        },
        // 他のオペコードのパターン
        _ => panic!("Unknown ARM Op!"),
    }
}

// TODO AARM命令の実行
pub fn arm_op_exec(_cpu: &mut CPU, _op: u32) {
    todo!("Plz! ARM ARM OP Exec Func Impl!")
}