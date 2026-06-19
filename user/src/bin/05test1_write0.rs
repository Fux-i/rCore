#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;
extern crate core;
use core::slice;
use user_lib::{STDOUT, write};

const STACK_SIZE: usize = 4096;

fn get_sp() -> usize {
    let mut sp: usize;
    unsafe {
        asm!("mv {}, sp", out(reg) sp);
    }
    (sp + STACK_SIZE - 1) & (!(STACK_SIZE - 1))
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    assert_eq!(
        write(STDOUT, unsafe {
            slice::from_raw_parts(0x80000000 as *const _, 10)
        }),
        -1
    );
    let sp = get_sp();
    assert_eq!(
        write(STDOUT, unsafe {
            slice::from_raw_parts((sp - 5) as *const _, 10)
        }),
        -1
    );
    // TODO: test string located in .data section
    println!("Test write0 OK!");
    0
}
