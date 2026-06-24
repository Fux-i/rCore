#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{
    SYSCALL_EXIT, SYSCALL_GET_TIME, SYSCALL_TRACE, SYSCALL_WRITE, SYSCALL_YIELD, count_syscall,
    get_time, println, sleep, trace_read, trace_write,
};

pub fn write_const(var: &u8, new_val: u8) {
    trace_write(var as *const _, new_val);
}

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let t1 = get_time() as usize;
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    let t3 = get_time() as usize;
    assert!(3 <= count_syscall(SYSCALL_GET_TIME));
    // 注意这次 sys_trace 调用本身也计入
    assert_eq!(2, count_syscall(SYSCALL_TRACE));
    assert_eq!(0, count_syscall(SYSCALL_WRITE));
    assert!(0 < count_syscall(SYSCALL_YIELD));
    assert_eq!(0, count_syscall(SYSCALL_EXIT));

    println!("string from task trace test\n");
    let t4 = get_time() as usize;
    let t5 = get_time() as usize;
    assert!(5 <= count_syscall(SYSCALL_GET_TIME));
    assert_eq!(7, count_syscall(SYSCALL_TRACE));
    assert_eq!(1, count_syscall(SYSCALL_WRITE));
    assert!(0 < count_syscall(SYSCALL_YIELD));
    assert_eq!(0, count_syscall(SYSCALL_EXIT));

    #[allow(unused_mut)]
    let mut var = 111u8;
    assert_eq!(Some(111), trace_read(&var as *const u8));
    write_const(&var, (t1 ^ t2 ^ t3 ^ t4 ^ t5) as u8);
    assert_eq!((t1 ^ t2 ^ t3 ^ t4 ^ t5) as u8, unsafe {
        core::ptr::read_volatile(&var)
    });

    assert!(None != trace_read(main as *const _));
    println!("Test trace OK!\n");
    0
}
