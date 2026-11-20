// SPDX-License-Identifier: BSD-2-Clause

use rustix::net::{AddressFamily, SocketFlags, SocketType, socketpair};

use systemi_platform::{
    ServiceFork, close_unneeded_fds,
    error::{Action, Error},
    fork_it, main_privileged, main_unprivileged,
};

fn main() -> Result<(), Error> {
    close_unneeded_fds()?;

    let (parent, child) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC,
        None,
    )
    .map_err(|err| Error {
        act: Action::Socket,
        err,
        msg: "socket pair for privileged/unprivileged communication",
    })?;

    // SAFETY: No standard libraries locks as used, as far as I can tell. The only resource created
    // is designed to be passed through the fork.
    match unsafe { fork_it(parent, child)? } {
        ServiceFork::Parent {
            helper,
            child: _child,
        } => main_privileged(helper)?,
        ServiceFork::Child { helper } => main_unprivileged(helper)?,
    }

    Ok(())
}
