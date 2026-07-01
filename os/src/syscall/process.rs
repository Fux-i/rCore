//! Process management syscalls

use crate::config::PAGE_SIZE;
use crate::mm::{MapPermission, VirtAddr};
use crate::task::{
    change_program_brk, exit_current_and_run_next, suspend_current_and_run_next,
    with_current_memory_set,
};
use crate::timer::get_time_ms;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get current time
pub fn sys_get_time() -> isize {
    get_time_ms() as isize
}

/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}

/// map memory
pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    if start % PAGE_SIZE != 0 || prot & !0x7 != 0 || prot & 0x7 == 0 {
        return -1;
    }
    if len == 0 {
        return 0;
    }
    let Some(end) = start.checked_add(len) else {
        return -1;
    };
    let mut permission = MapPermission::U;
    if prot & 0x1 != 0 {
        permission |= MapPermission::R;
    }
    if prot & 0x2 != 0 {
        permission |= MapPermission::W;
    }
    if prot & 0x4 != 0 {
        permission |= MapPermission::X;
    }
    with_current_memory_set(|memory_set| {
        if memory_set.mmap(VirtAddr::from(start), VirtAddr::from(end), permission) {
            0
        } else {
            -1
        }
    })
}

/// unmap memory
pub fn sys_munmap(start: usize, len: usize) -> isize {
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    if len == 0 {
        return 0;
    }
    let Some(end) = start.checked_add(len) else {
        return -1;
    };
    with_current_memory_set(|memory_set| {
        if memory_set.munmap(VirtAddr::from(start), VirtAddr::from(end)) {
            0
        } else {
            -1
        }
    })
}
