// SPDX-License-Identifier: BSD-2-Clause

pub mod sandbox;

pub mod real_main;
pub use real_main::{fork_it, main_privileged, main_unprivileged};
