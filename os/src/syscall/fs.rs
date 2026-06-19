//! File and filesystem-related syscalls
use log::warn;

use crate::batch::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, USER_STACK, USER_STACK_SIZE};

const FD_STDOUT: usize = 1;

fn check_addr_legality(slice: &[u8]) -> Option<isize> {
    let user_stack_base = USER_STACK.get_sp();
    let start = slice.as_ptr().addr();
    let len = slice.len();
    if start >= APP_BASE_ADDRESS && start + len <= APP_BASE_ADDRESS + APP_SIZE_LIMIT
        || start >= user_stack_base - USER_STACK_SIZE && start + len <= user_stack_base
    {
        Some(len as isize)
    } else {
        warn!("[sys_write safety check] illegal address!");
        None
    }
}

/// write buf of length `len`  to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            match check_addr_legality(slice) {
                None => -1 as isize,
                Some(i_len) => {
                    let str = core::str::from_utf8(slice).unwrap();
                    print!("{}", str);
                    i_len
                }
            }
        }
        _ => {
            warn!("Unsupported fd in sys_write!");
            -1
        }
    }
}
