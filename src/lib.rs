// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! ![Build Status](https://github.com/dusk-network/dusk-abi/workflows/Continuous%20integration/badge.svg)
//! [![Repository](https://img.shields.io/badge/github-dusk--abi-blueviolet?logo=github)](https://github.com/dusk-network/dusk-abi)
//! [![Documentation](https://img.shields.io/badge/docs-dusk--abi-blue?logo=rust)](https://docs.rs/dusk-abi/)

//! # Dusk ABI
//!
//! The official ABI to develop dusk-network contracts and host module for Rusk
//! VM
#![warn(missing_docs)]
#![no_std]
#![feature(core_intrinsics, lang_items, alloc_error_handler)]

mod types;
pub use types::contract::{ContractId, ContractState};
pub use types::query::Query;
pub use types::return_value::ReturnValue;
pub use types::transaction::Transaction;

/// The trait that host modules uses to defines their own ID for both host and
/// hosted environment
pub trait Module {
    /// Returns the id that identifies this module
    fn id() -> ContractId
    where
        Self: Sized;

    /// Returns the module's id from the object instance
    fn module_id(&self) -> ContractId
    where
        Self: Sized,
    {
        Self::id()
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        // re-export WeeAlloc
        pub use wee_alloc::WeeAlloc;

        #[doc(hidden)]
        pub mod hosted;
        pub use hosted::*;
    } else {
        #[doc(hidden)]
        pub mod host;
        pub use host::*;
    }
}
