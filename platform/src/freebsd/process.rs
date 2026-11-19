// SPDX-License-Identifier: BSD-2-Clause

use core::{hint::cold_path, mem::MaybeUninit};

use libc::{PD_CLOEXEC, pdfork};
use rustix::{
    fd::{FromRawFd, OwnedFd},
    process::{RawPid, Signal, set_parent_process_death_signal},
};

use crate::{
    error::{Action, Error},
    platform::common::{ServiceFork, gib_errno},
};

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
