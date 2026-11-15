// SPDX-License-Identifier: BSD-2-Clause

pub mod platform;

use rustix::{
    io::Errno,
    net::{AddressFamily, SocketFlags, SocketType, socketpair},
};

use crate::platform::{common::ServiceFork, fork_it, main_privileged, main_unprivleged};

fn main() -> Result<(), Errno> {
    let (parent, child) = socketpair(
        AddressFamily::UNIX,
        SocketType::STREAM,
        SocketFlags::CLOEXEC,
        None,
    )?;

    // SAFETY: No standard libraries locks as used, as far as I can tell. The only resource created
    // is designed to be passed through the fork.
    match unsafe { fork_it(parent, child)? } {
        ServiceFork::Parent { helper, child } => {}
        ServiceFork::Child { helper } => {}
    }

    Ok(())
}
