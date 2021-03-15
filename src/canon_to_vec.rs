// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

extern crate alloc;

use alloc::vec::Vec;
use canonical::{Canon, Sink};

pub(crate) trait CanonToVec {
    fn encode_to_vec(&self) -> Vec<u8>;
}

impl<T> CanonToVec for T
where
    T: Canon,
{
    fn encode_to_vec(&self) -> Vec<u8> {
        let len = self.encoded_len();

        let mut vec = Vec::with_capacity(len);
        vec.resize_with(len, || 0);
        let mut sink = Sink::new(&mut vec[..]);

        self.encode(&mut sink);
        vec
    }
}
