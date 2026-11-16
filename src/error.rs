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
    Open,
    Close,
    Dup,
    Fcntl,
    Fork,
    Procctl,
    Socket,
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Action::Open => write!(f, "open"),
            Action::Close => write!(f, "close"),
            Action::Dup => write!(f, "dup"),
            Action::Fcntl => write!(f, "fcntl"),
            Action::Fork => write!(f, "fork"),
            Action::Procctl => write!(f, "procctl"),
            Action::Socket => write!(f, "socket"),
        }
    }
}
