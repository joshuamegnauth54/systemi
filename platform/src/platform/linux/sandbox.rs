// SPDX-License-Identifier: BSD-2-Clause

use libc::{gid_t, uid_t};
use rustix::{
    io::Errno,
    thread::{
        CapabilitySet, CapabilitySets, capabilities, clear_ambient_capability_set,
        remove_capability_from_bounding_set, set_capabilities, set_keep_capabilities,
        set_no_new_privs,
    },
};

use crate::{
    error::{Action, Error},
    platform::common::{set_gid, set_uid, unset_groups},
};

const DROP_AMBIENT: &str = "Dropping ambient capabilities (PR_CAP_AMBIENT_CLEAR_ALL)";
const CAPBSET_DROP: &str = "Dropping bounding set capabilities (PR_CAPBSET_DROP)";
const NO_NEW_PRIVS: &str = "Enabling no new privileges (PR_SET_NO_NEW_PRIVS)";
const CAPABILITIES: &str = "Checking permitted capabilities";

pub fn sandbox_unprivileged(uid: uid_t, gid: gid_t) -> Result<(), Error> {
    if uid == 0 || gid == 0 {
        return Err(Error {
            act: Action::Sanity,
            err: Errno::INVAL,
            msg: "Unprivileged daemon requested to be run as root",
        });
    }

    clear_ambient_capability_set().map_err(|err| Error::procctl(err, DROP_AMBIENT))?;
    drop_bounding(CapabilitySet::empty())?;

    unset_groups()?;
    set_gid(gid)?;
    set_keep_capabilities(true).map_err(|err| {
        Error::procctl(err, "Rejecting permissions across setuid (PR_SET_KEEPCAPS)")
    })?;
    set_uid(uid)?;
    set_no_new_privs(true).map_err(|err| Error::procctl(err, NO_NEW_PRIVS))?;

    Ok(())
}

pub fn sandbox_privileged(uid: uid_t, gid: gid_t, cap: CapabilitySet) -> Result<(), Error> {
    // XXX: If root is disallowed, check it in the function that calls this function in the daemon.
    // This is subject to change if all of the Linux implementations can work without root.
    if !capabilities(None)
        .map_err(|err| Error {
            act: Action::CapGet,
            err,
            msg: CAPABILITIES,
        })?
        .permitted
        .contains(cap)
    {
        return Err(Error {
            act: Action::Sanity,
            err: Errno::PERM,
            msg: "Process lacks required permissions",
        });
    }

    clear_ambient_capability_set().map_err(|err| Error::procctl(err, DROP_AMBIENT))?;
    drop_bounding(cap)?;
    unset_groups()?;
    set_gid(gid)?;

    // Permissions need to be explicitly kept across setuid.
    set_keep_capabilities(true).map_err(|err| {
        Error::procctl(
            err,
            "Preserving permissions across setuid (PR_SET_KEEPCAPS)",
        )
    })?;
    set_uid(uid)?;

    let expected = CapabilitySets {
        effective: cap,
        permitted: cap,
        inheritable: CapabilitySet::empty(),
    };
    set_capabilities(None, expected).map_err(|err| Error {
        act: Action::CapSet,
        err,
        msg: "Setting expected permissions across setuid",
    })?;
    set_no_new_privs(true).map_err(|err| Error::procctl(err, NO_NEW_PRIVS))?;

    let actual = capabilities(None).map_err(|err| Error {
        act: Action::CapGet,
        err,
        msg: CAPABILITIES,
    })?;
    if actual == expected {
        Ok(())
    } else {
        Err(Error {
            act: Action::Sanity,
            err: Errno::PERM,
            msg: "Invalid process capabilities",
        })
    }
}

/// Drop all bounding capabilities except those in the set.
pub fn drop_bounding(except: CapabilitySet) -> Result<(), Error> {
    for cap in CapabilitySet::all()
        .iter()
        .filter(|&cap| !except.contains(cap))
    {
        if let Err(err) = remove_capability_from_bounding_set(cap)
            && err != Errno::INVAL
        {
            return Err(Error::procctl(err, CAPBSET_DROP));
        }
    }

    Ok(())
}
