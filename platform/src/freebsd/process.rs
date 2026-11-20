// SPDX-License-Identifier: BSD-2-Clause

use core::{hint::cold_path, mem::MaybeUninit};

use libc::{PD_CLOEXEC, pdfork};
use rustix::{
    fd::{FromRawFd, OwnedFd},
    process::{RawPid, Signal, set_parent_process_death_signal},
};

use crate::{
    common::{ServiceFork, gib_errno},
    error::{Action, Error},
};

/// Fork, ensuring child dies with the parent.
///
/// # Safety
/// * Global state is left untouched before fork to prevent deadlocks or invalid state.
/// * Inherited file descriptors should be in a neutral state to prevent weird interactions.
///   Currently, the daemons are designed to close all file descriptors other than the helper and
///   standard streams.
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn fork_it(parent: OwnedFd, child: OwnedFd) -> Result<ServiceFork, Error> {
    let mut pidfd = MaybeUninit::<RawPid>::uninit();

    match pdfork(pidfd.as_mut_ptr(), PD_CLOEXEC) {
        -1 => {
            cold_path();
            Err(Error {
                act: Action::Fork,
                err: gib_errno(),
                msg: "Forking into child",
            })
        }
        0 => {
            // Child is killed on drop, so this is just some extra assurance for panic = "abort".
            set_parent_process_death_signal(Some(Signal::KILL))
                .map_err(|err| Error::procctl(err, "Setting parent death signal"))?;
            Ok(ServiceFork::Child { helper: child })
        }
        pid => {
            let pidfd = pidfd.assume_init();
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
