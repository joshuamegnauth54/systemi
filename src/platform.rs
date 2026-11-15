// SPDX-License-Identifier: BSD-2-Clause

#[cfg(target_os = "freebsd")]
mod freebsd;

#[cfg(target_os = "freebsd")]
pub use freebsd::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(any(target_os = "freebsd", target_os = "linux"))]
mod freebsd_linux;

pub mod common {
    use rustix::fd::OwnedFd;

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
}
