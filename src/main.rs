mod cpu;
mod bus;
mod common;
mod thumb_op;
mod arm_op;

extern crate bitflags;
extern crate bitvec;

#[macro_use]
extern crate log;
extern crate env_logger;

fn main() {
    println!("Hello, World!");
}