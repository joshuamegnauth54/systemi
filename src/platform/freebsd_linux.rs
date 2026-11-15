// SPDX-License-Identifier: BSD-2-Clause

use core::mem::DropGuard;

use libc::close_range;
use rustix::{
    fd::{AsRawFd, BorrowedFd, IntoRawFd, OwnedFd},
    fs::{Mode, OFlags, open},
    io::{DupFlags, Errno, dup3, fcntl_getfd},
    stdio::{raw_stderr, raw_stdin, take_stdin},
};

#[cfg(target_os = "freebsd")]
use libc::__error;

#[cfg(target_os = "linux")]
use libc::__errno_location as __error;

/// Close stdin and all file descriptors other than stdout and stderr.
pub fn close_unneeded_fds() -> Result<(), Errno> {
    let null = open(
        c"/dev/null",
        OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::PATH | OFlags::RDWR,
        Mode::empty(),
    )?;

    // If null is already 0, then stdin was already closed and now stdin points to null.
    if null.as_raw_fd() == raw_stdin() {
        // Ensure it's not dropped or else we'd just close stdin again.
        _ = null.into_raw_fd();
        return Ok(());
    }

    // Dup stdin onto /dev/null and ensure it's not closed on drop.
    // (Exciting that I found a use for DropGuard a day after release!)
    fcntl_getfd(unsafe { BorrowedFd::borrow_raw(raw_stdin()) })?;
    let mut stdin = unsafe {
        DropGuard::new(take_stdin(), |stdin| {
            _ = stdin.into_raw_fd();
        })
    };
    dup3(&null, &mut stdin, DupFlags::CLOEXEC)?;

    // TODO: likely_unlikely
    if unsafe { close_range(raw_stderr() as u32 + 1, u32::MAX, 0) } == -1 {
        Err(Errno::from_raw_os_error(unsafe { *__error() }))
    } else {
        Ok(())
    }
}
