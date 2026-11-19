// SPDX-License-Identifier: BSD-2-Clause

use core::hint::cold_path;

use libc::{gid_t, uid_t};
use rustix::io::Errno;

use crate::{
    common::gib_errno,
    error::{Action, Error},
};

pub fn sandbox_unprivileged(uid: uid_t, gid: gid_t) -> Result<(), Error> {
    if uid == 0 || gid == 0 {
        return Err(Error {
            act: Action::Sanity,
            err: Errno::INVAL,
            msg: "Unprivileged daemon requested to be run as root",
        });
    }

    unset_groups()?;
    set_gid(gid)?;
    set_uid(uid)?;

    Ok(())
}


pub fn capsicum() -> Result<(), Error> {
    if unsafe { libc::cap_enter() } == 0 {
        Ok(())
    } else {
        cold_path();
        Err(Error {
            act: Action::CapEnter,
            err: gib_errno(),
            msg: "Entering capability mode",
        })
    }
}
