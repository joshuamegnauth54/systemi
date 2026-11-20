// SPDX-License-Identifier: BSD-2-Clause

use core::mem::DropGuard;

use libc::close_range;
use rustix::{
    fd::{AsRawFd, BorrowedFd, IntoRawFd},
    fs::{Mode, OFlags, open},
    io::{DupFlags, dup3, fcntl_getfd},
    stdio::{raw_stderr, raw_stdin, take_stdin},
};

use crate::{
    common::gib_errno,
    error::{Action, Error},
};

/// Close stdin and all file descriptors other than stdout and stderr.
///
/// This is NOT aware of any file descriptors opened before calling this function. Take heed of this
/// if you need other file descriptors that you have opened prior.
pub fn close_unneeded_fds() -> Result<(), Error> {
    let null = open(
        c"/dev/null",
        OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::PATH | OFlags::RDONLY,
        Mode::empty(),
    )
    .map_err(|err| Error {
        act: Action::Open,
        err,
        msg: "opening /dev/null to dup stdin onto",
    })?;

    // If null is already 0, then stdin was already closed and now stdin points to null.
    if null.as_raw_fd() == raw_stdin() {
        // Ensure it's not dropped or else we'd just close stdin again.
        _ = null.into_raw_fd();
    } else {
        // Dup stdin onto /dev/null and ensure it's not closed on drop.
        // (Exciting that I found a use for DropGuard a day after release!)
        fcntl_getfd(unsafe { BorrowedFd::borrow_raw(raw_stdin()) }).map_err(|err| Error {
            act: Action::Fcntl,
            err,
            msg: "F_GETFD on stdin to check if it's open",
        })?;
        let mut stdin = unsafe {
            DropGuard::new(take_stdin(), |stdin| {
                _ = stdin.into_raw_fd();
            })
        };
        dup3(&null, &mut stdin, DupFlags::CLOEXEC).map_err(|err| Error {
            act: Action::Dup,
            err,
            msg: "dup stdin -> /dev/null",
        })?;
    }

    // TODO: likely_unlikely
    if unsafe { close_range(raw_stderr() as u32 + 1, u32::MAX, 0) } == -1 {
        Err(Error {
            act: Action::Close,
            err: gib_errno(),
            msg: "closing all file descriptors above stderr",
        })
    } else {
        Ok(())
    }
}
