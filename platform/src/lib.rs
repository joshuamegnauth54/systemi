// SPDX-License-Identifier: BSD-2-Clause

mod common;
pub use common::*;

pub mod error;

#[cfg(target_os = "freebsd")]
mod freebsd;

#[cfg(target_os = "freebsd")]
pub use freebsd::*;

#[cfg(any(target_os = "freebsd", target_os = "linux"))]
mod freebsd_linux;

#[cfg(any(target_os = "freebsd", target_os = "linux"))]
pub use freebsd_linux::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;
