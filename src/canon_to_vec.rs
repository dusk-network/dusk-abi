// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.
extern crate alloc;

use alloc::vec::Vec;
use canonical::{ByteSink, Canon, Store};

pub(crate) trait CanonToVec<S>
where
    S: Store,
{
    fn encode_to_vec(&self, store: &S) -> Result<Vec<u8>, S::Error>;
}

impl<T, S> CanonToVec<S> for T
where
    T: Canon<S>,
    S: Store,
{
    fn encode_to_vec(&self, store: &S) -> Result<Vec<u8>, S::Error> {
        let len = Canon::<S>::encoded_len(self);

        let mut vec = Vec::new();
        vec.resize_with(len, || 0);
        let mut sink = ByteSink::new(&mut vec[..], store);

        Canon::<S>::write(self, &mut sink)?;
        Ok(vec)
    }
}
