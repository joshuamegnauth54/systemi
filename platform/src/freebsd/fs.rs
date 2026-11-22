// SPDX-License-Identifier: BSD-2-Clause

use core::ffi::CStr;

use rustix::{
    fd::{BorrowedFd, OwnedFd},
    fs::{Mode, OFlags, ResolveFlags, openat},
};

use crate::error::{Action, Error};

#[inline]
pub fn resolve_beneath(
    dfd: BorrowedFd<'_>,
    path: &CStr,
    flags: OFlags,
    mode: Mode,
    msg: &'static str,
) -> Result<OwnedFd, Error> {
    openat(
        dfd,
        path,
        flags | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::RESOLVE_BENEATH,
        mode,
    )
    .map_err(|err| Error {
        act: Action::Open,
        err,
        msg,
    })
}
