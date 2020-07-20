// SPDX-FileCopyrightText: © 2020 Etebase Authors
// SPDX-License-Identifier: LGPL-2.1-only

pub mod crypto;
pub mod service;
pub mod content;
pub mod utils;
pub mod error;

pub use error::Error;

pub fn init() -> error::Result<()> {
    crypto::init()
}

mod capi;
