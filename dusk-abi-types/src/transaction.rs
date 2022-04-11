// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

extern crate alloc;

use alloc::vec::Vec;
use canonical::{Canon, CanonError, EncodeToVec, Source};
use canonical_derive::Canon;

/// A generic transaction
#[derive(Clone, Canon, Debug, Default)]
pub struct Transaction(Vec<u8>);

impl Transaction {
    /// Returns the byte representation of the transaction
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    /// Creates a transaction from a raw bytes
    pub fn from_slice(buffer: &[u8]) -> Self {
        Transaction(buffer.to_vec())
    }

    /// Creates a transaction from a type implementing `Canon`
    pub fn from_canon<C>(c: &C) -> Self
    where
        C: Canon,
    {
        Transaction(c.encode_to_vec())
    }

    /// Casts the generic query to given type
    pub fn cast<C>(&self) -> Result<C, CanonError>
    where
        C: Canon,
    {
        let mut source = Source::new(self.as_bytes());
        C::decode(&mut source)
    }
}
