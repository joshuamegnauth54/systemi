// SPDX-License-Identifier: BSD-2-Clause

use core::{
    error,
    fmt::{self, Display, Formatter},
};

use rustix::io::Errno;

#[derive(Clone, Copy, Debug)]
pub struct Error {
    pub act: Action,
    pub err: Errno,
    pub msg: &'static str,
}

impl Error {
    #[cold]
    #[inline(never)]
    pub const fn procctl(err: Errno, msg: &'static str) -> Self {
        Self {
            act: Action::Procctl,
            err,
            msg,
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]: {}", self.act, self.err, self.msg)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    #[cfg(target_os = "linux")]
    CapGet,
    #[cfg(target_os = "linux")]
    CapSet,
    Close,
    Dup,
    Fcntl,
    Fork,
    Open,
    Procctl,
    Sanity,
    SetGid,
    SetGroups,
    SetUid,
    Socket,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            #[cfg(target_os = "linux")]
            Action::CapGet => write!(f, "capget"),
            #[cfg(target_os = "linux")]
            Action::CapSet => write!(f, "capset"),
            Action::Close => write!(f, "close"),
            Action::Dup => write!(f, "dup"),
            Action::Fcntl => write!(f, "fcntl"),
            Action::Fork => write!(f, "fork"),
            Action::Open => write!(f, "open"),
            #[cfg(not(target_os = "linux"))]
            Action::Procctl => write!(f, "procctl"),
            #[cfg(target_os = "linux")]
            Action::Procctl => write!(f, "prctl"),
            Action::Sanity => write!(f, "sanity"),
            Action::SetGid => write!(f, "setgid"),
            Action::SetGroups => write!(f, "setgroups"),
            Action::SetUid => write!(f, "setuid"),
            Action::Socket => write!(f, "socket"),
        }
    }
}
