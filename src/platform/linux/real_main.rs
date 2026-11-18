// SPDX-License-Identifier: BSD-2-Clause

use core::{
    hint::cold_path,
    mem::{MaybeUninit, size_of},
};

use libc::{CLONE_PIDFD, SYS_clone3, clone_args, syscall};
use rustix::{
    fd::{FromRawFd, OwnedFd},
    process::{RawPid, Signal, set_parent_process_death_signal},
};

use crate::{
    error::{Action, Error},
    platform::common::{ServiceFork, gib_errno},
};

#[expect(unsafe_op_in_unsafe_fn)]
pub unsafe fn fork_it(parent: OwnedFd, child: OwnedFd) -> Result<ServiceFork, Error> {
    let mut pidfd = MaybeUninit::<RawPid>::uninit();
    let mut args = clone_args {
        flags: CLONE_PIDFD as u64,
        pidfd: pidfd.as_mut_ptr().expose_provenance() as u64,
        child_tid: 0,
        parent_tid: 0,
        exit_signal: Signal::CHILD.as_raw() as u64,
        stack: 0,
        stack_size: 0,
        tls: 0,
        set_tid: 0,
        set_tid_size: 0,
        cgroup: 0,
    };

    match syscall(SYS_clone3, &raw mut args, size_of::<clone_args>()) {
        -1 => {
            cold_path();
            Err(Error {
                act: Action::Fork,
                err: gib_errno(),
                msg: "forking into child with CLONE3",
            })
        }
        0 => {
            set_parent_process_death_signal(Some(Signal::KILL))
                .map_err(|err| Error::procctl(err, "Setting parent death signal"))?;

            Ok(ServiceFork::Child { helper: child })
        }
        _pid => {
            // Like FreeBSD's path, I want a PID file descriptor.
            let pidfd = pidfd.assume_init();
            // Child intentionally shadowed here.
            let child = OwnedFd::from_raw_fd(pidfd);
            Ok(ServiceFork::Parent {
                helper: parent,
                child,
            })
        }
    }
}

pub fn main_privileged(helper: OwnedFd) -> Result<(), Error> {
    todo!()
}

pub fn main_unprivileged(helper: OwnedFd) -> Result<(), Error> {
    todo!()
}
