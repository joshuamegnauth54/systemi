// SPDX-License-Identifier: BSD-2-Clause

pub mod buffer;

use core::{hint::cold_path, num::NonZeroU32, ptr};

use libc::{setgid, setgroups, setuid};
use rustix::{
    fd::{BorrowedFd, OwnedFd},
    io::{Errno, read},
};

use crate::error::{Action, Error};

#[cfg(any(target_os = "freebsd", target_vendor = "apple"))]
use libc::__error;

#[cfg(any(target_os = "helenos", target_os = "netbsd", target_os = "openbsd",))]
use libc::__errno as __error;

#[cfg(any(
    target_os = "dragonfly",
    target_os = "fuchsia",
    target_os = "hurd",
    target_os = "linux",
    target_os = "redox",
))]
use libc::__errno_location as __error;

#[cfg(any(target_os = "illumos", target_os = "solaris"))]
use libc::___errno as __error;

/// Result of forking into unprivileged D-Bus service.
///
/// The parent owns one half of a socket to communicate with the child while the child owns the
/// other half. The parent also owns a process file descriptor for which to control the child
/// with pdkill(2) on FreeBSD or pidfd_send_signal(2) and friends on Linux.
pub enum ServiceFork {
    Parent { helper: OwnedFd, child: OwnedFd },
    Child { helper: OwnedFd },
}

#[cold]
#[inline(never)]
pub fn gib_errno() -> Errno {
    // SAFETY: This is not unsafe.
    Errno::from_raw_os_error(unsafe { *__error() })
}

pub fn unset_groups() -> Result<(), Error> {
    if unsafe { setgroups(0, ptr::null()) } == -1 {
        cold_path();
        Err(Error {
            act: Action::SetGroups,
            err: gib_errno(),
            msg: "Dropping supplementary groups",
        })
    } else {
        Ok(())
    }
}

pub fn set_gid(gid: NonZeroU32) -> Result<(), Error> {
    if unsafe { setgid(gid.get()) } == 0 {
        Ok(())
    } else {
        cold_path();
        Err(Error {
            act: Action::SetGid,
            err: gib_errno(),
            msg: "Dropping real, effective, and set group ID",
        })
    }
}

pub fn set_uid(uid: NonZeroU32) -> Result<(), Error> {
    if unsafe { setuid(uid.get()) } == 0 {
        Ok(())
    } else {
        cold_path();
        Err(Error {
            act: Action::SetUid,
            err: gib_errno(),
            msg: "Dropping real, effective, and set user ID",
        })
    }
}

/// Read until EOF or buf is full.
pub fn read_buf<'b>(
    fd: BorrowedFd<'_>,
    buf: &'b mut [u8],
    msg: &'static str,
) -> Result<&'b [u8], Error> {
    let mut nread = 0;

    loop {
        match read(fd, &mut buf[nread..]) {
            Ok(n) => {
                nread += n;
                if n == 0 || nread == buf.len() {
                    break;
                }
            }
            Err(Errno::INTR) => (),
            Err(err) => {
                return Err(Error {
                    act: Action::Read,
                    err,
                    msg,
                });
            }
        }
    }

    Ok(&buf[..nread])
}
