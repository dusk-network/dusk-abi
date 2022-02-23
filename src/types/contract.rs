// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

extern crate alloc;

use alloc::vec::Vec;
use canonical::{Canon, CanonError, EncodeToVec, Source};
use canonical_derive::Canon;

/// Bytes representing a contract state
#[derive(Clone, Canon, Debug, Default)]
pub struct ContractState(Vec<u8>);

impl ContractState {
    /// Returns the state of the contract as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    /// Creates a state from a type implementing `Canon`
    pub fn from_canon<C>(c: &C) -> Self
    where
        C: Canon,
    {
        ContractState(c.encode_to_vec())
    }

    /// Casts the encoded state to given type
    pub fn cast<C>(&self) -> Result<C, CanonError>
    where
        C: Canon,
    {
        let mut source = Source::new(self.as_bytes());
        C::decode(&mut source)
    }
}

/// Type used to identify a contract
#[derive(Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Canon)]
pub struct ContractId([u8; 32]);

impl<B> From<B> for ContractId
where
    B: AsRef<[u8]>,
{
    fn from(b: B) -> Self {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(b.as_ref());
        ContractId(bytes)
    }
}

impl ContractId {
    /// Return a reserved contract id for host fn modules
    pub const fn reserved(id: u8) -> Self {
        let mut bytes = [0; 32];
        bytes[0] = id;
        ContractId(bytes)
    }

    /// Returns the contract id as a byte slice
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Returns the contract id as a mutable slice
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }

    /// Returns a `ContractId` from an array of 32 bytes
    pub const fn from_raw(b: [u8; 32]) -> Self {
        Self(b)
    }
}

impl core::fmt::LowerHex for ContractId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.as_bytes();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in bytes {
            write!(f, "{:02x}", &byte)?
        }

        Ok(())
    }
}

impl core::fmt::UpperHex for ContractId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let bytes = self.as_bytes();

        if f.alternate() {
            write!(f, "0x")?
        }

        for byte in bytes {
            write!(f, "{:02X}", &byte)?
        }

        Ok(())
    }
}

impl core::fmt::Display for ContractId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(self, f)
    }
}

#[allow(non_snake_case)]
impl core::fmt::Debug for ContractId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Once we format an object using the debug notation (e.g. `{:x?}`)
        // there is absolutely NO WAY to detect the flag for the lowerhex
        // or upperhex, and therefore forwarding to the relevant formatter.
        // Two methods for this purpose exists, but they're not exposed
        // because they didn't agree on a name yet, see:
        // <https://github.com/rust-lang/rust/blob/90442458ac46b1d5eed752c316da25450f67285b/library/core/src/fmt/mod.rs#L1817-L1825>
        //
        // Therefore the only way is using the deprecated method `flags`,
        // implementing the same logic of the forementioned methods.

        // We also do not have access to the `FlagV1` enum since it's
        // private.
        let FlagV1_DebugUpperHex = 5_u32;

        #[allow(deprecated)]
        if f.flags() & (1 << FlagV1_DebugUpperHex) != 0 {
            core::fmt::UpperHex::fmt(self, f)
        } else {
            // LowerHex is always the default for debug
            core::fmt::LowerHex::fmt(self, f)
        }
    }
}
