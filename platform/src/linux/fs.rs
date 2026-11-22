// SPDX-License-Identifier: BSD-2-Clause

use core::ffi::CStr;

use rustix::{
    fd::{BorrowedFd, OwnedFd},
    fs::{Mode, OFlags, ResolveFlags, openat2},
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
    openat2(
        dfd,
        path,
        flags | OFlags::CLOEXEC | OFlags::NOFOLLOW,
        mode,
        ResolveFlags::BENEATH
            | ResolveFlags::NO_SYMLINKS
            | ResolveFlags::NO_SYMLINKS
            | ResolveFlags::NO_XDEV,
    )
    .map_err(|err| Error {
        act: Action::Open,
        err,
        msg,
    })
}
