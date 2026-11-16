// SPDX-License-Identifier: BSD-2-Clause

use core::{hint::cold_path, mem::MaybeUninit};

use libc::{PD_CLOEXEC, pdfork};
use rustix::{
    fd::{FromRawFd, OwnedFd},
    io::Errno,
    process::RawPid,
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
            let err = Errno::from_raw_os_error(gib_errno());
            Err(Error {
                act: Action::Fork,
                err,
                msg: "Forking into child",
            })
        }
        0 => Ok(ServiceFork::Child { helper: child }),
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

pub fn main_privileged(helper: OwnedFd) -> Result<(), Errno> {
    todo!()
}

pub fn main_unprivleged(helper: OwnedFd) -> Result<(), Errno> {
    todo!()
}
