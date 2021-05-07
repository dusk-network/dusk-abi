// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

extern crate alloc;

use alloc::vec::Vec;
use canonical::{Canon, CanonError, EncodeToVec, Source};
use canonical_derive::Canon;

/// A generic return value
#[derive(Clone, Canon, Debug, Default, PartialEq)]
pub struct ReturnValue(Vec<u8>);

impl ReturnValue {
    /// Returns the byte representation of the return value
    pub fn as_bytes(&self) -> &[u8] {
        &self.0[..]
    }

    /// Creates a return value from a raw bytes
    pub fn from_slice(buffer: &[u8]) -> Self {
        ReturnValue(buffer.to_vec())
    }

    /// Creates a return value from a type implementing `Canon`
    pub fn from_canon<C>(c: &C) -> Self
    where
        C: Canon,
    {
        ReturnValue(c.encode_to_vec())
    }

    /// Casts the encoded return value to given type
    pub fn cast<C>(&self) -> Result<C, CanonError>
    where
        C: Canon,
    {
        let mut source = Source::new(self.as_bytes());
        C::decode(&mut source)
    }
}
