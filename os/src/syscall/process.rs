//! Process management syscalls
use log::trace;

use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::get_time_ms;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("[kernel] App yielded cpu");
    suspend_current_and_run_next();
    0
}

/// get time in milliseconds
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    unsafe {
        match _trace_request {
            0 => (_id as *const isize).read(),
            1 => {
                (_id as *mut u8).write_bytes(_data as u8, 1);
                0
            }
            2 => crate::task::get_count(super::map_index(_id)),
            _ => -1,
        }
    }
}
