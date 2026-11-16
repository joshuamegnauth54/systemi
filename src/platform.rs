// SPDX-License-Identifier: BSD-2-Clause

pub mod common;

#[cfg(target_os = "freebsd")]
mod freebsd;

#[cfg(target_os = "freebsd")]
pub use freebsd::*;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(any(target_os = "freebsd", target_os = "linux"))]
mod freebsd_linux;
