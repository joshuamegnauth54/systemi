// SPDX-License-Identifier: BSD-2-Clause

use core::ffi::c_int;

use rustix::fd::OwnedFd;

#[cfg(any(target_os = "freebsd", target_vendor = "apple"))]
use libc::__error;

#[cfg(any(
    target_os = "helenos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
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

#[cfg(any(target_os = "freebsd", target_os = "linux"))]
pub use super::freebsd_linux::*;

/// Result of forking into unprivileged D-Bus service.
///
/// The parent owns one half of a socket to communicate with the child while the child owns the
/// other half. The parent also owns a process file descriptor for which to control the child
/// with pdkill(2) on FreeBSD or pidfd_send_signal(2) and friends on Linux.
pub enum ServiceFork {
    Parent { helper: OwnedFd, child: OwnedFd },
    Child { helper: OwnedFd },
}

#[inline]
pub fn gib_errno() -> c_int {
    // SAFETY: This is not unsafe.
    unsafe { *__error() }
}
