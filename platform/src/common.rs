// SPDX-License-Identifier: BSD-2-Clause

use core::{hint::cold_path, num::NonZeroU32, ptr};

use libc::{setgid, setgroups, setuid};
use rustix::{fd::OwnedFd, io::Errno};

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
    if unsafe { setgroups(0, ptr::null()) } == 0 {
        Ok(())
    } else {
        cold_path();
        Err(Error {
            act: Action::SetGroups,
            err: gib_errno(),
            msg: "Dropping supplementary groups",
        })
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
