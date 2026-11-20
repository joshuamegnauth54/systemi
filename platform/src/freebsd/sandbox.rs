// SPDX-License-Identifier: BSD-2-Clause

use core::{hint::cold_path, num::NonZeroU32};

use libc::{geteuid, gid_t, uid_t};
use rustix::{io::Errno, process::geteuid};

use crate::{
    common::{gib_errno, set_gid, set_uid, unset_groups},
    error::{Action, Error},
};

pub fn sandbox_unprivileged(uid: NonZeroU32, gid: NonZeroU32) -> Result<(), Error> {
    unset_groups()?;
    set_gid(gid)?;
    set_uid(uid)?;

    Ok(())
}

pub fn sandbox_privileged(uid: Option<NonZeroU32>, gid: Option<NonZeroU32>) -> Result<(), Error> {
    unset_groups()?;

    if let Some(uid) = uid {
        set_uid(uid)?;
    } else if !geteuid().is_root() {
        return Err(Error {
            act: Action::Sanity,
            err: Errno::PERM,
            msg: "Privileged daemon is not running as root",
        });
    }
    if let Some(gid) = gid {
        set_gid(gid)?;
    }

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
