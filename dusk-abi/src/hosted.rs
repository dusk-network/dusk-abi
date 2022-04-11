// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

const BUFFER_SIZE_LIMIT: usize = 1024 * 16;

use canonical::{Canon, CanonError, Sink, Source};

pub use crate::{ContractId, ContractState, Query, ReturnValue, Transaction};

#[doc(hidden)]
pub mod panic_include;

#[doc(hidden)]
pub mod bufwriter;

#[doc(hidden)]
pub mod debug;

// declare available host-calls
pub mod external {
    extern "C" {
        #[allow(unused)]
        pub fn debug(buffer: &u8, len: i32);

        pub fn query(target: &u8, buf: &mut u8, gas_limit: u64);
        pub fn transact(target: &u8, buf: &mut u8, gas_limit: u64);

        pub fn caller(buffer: &mut u8);
        pub fn callee(buffer: &mut u8);

        pub fn gas(value: i32);
        pub fn gas_consumed() -> u64;
        pub fn gas_left() -> u64;
        pub fn block_height() -> u64;
    }
}

/// Returns the caller of the contract
pub fn caller() -> ContractId {
    let mut result = ContractId::default();
    unsafe { external::caller(&mut result.as_bytes_mut()[0]) }
    result
}

/// Returns the hash of the currently executing contract
pub fn callee() -> ContractId {
    let mut result = ContractId::default();
    unsafe { external::callee(&mut result.as_bytes_mut()[0]) }
    result
}

/// Returns the current block height
pub fn block_height() -> u64 {
    unsafe { external::block_height() }
}

/// Deduct a specified amount of gas from the call
pub fn gas(value: i32) {
    unsafe { external::gas(value) }
}

/// Return the amount of gas consumed until the point when the host call is
/// executed.
pub fn gas_consumed() -> u64 {
    unsafe { external::gas_consumed() }
}

/// Return the ammunt of gas left until the point when the host call is
/// executed.
pub fn gas_left() -> u64 {
    unsafe { external::gas_left() }
}

/// Call another contract at address `target`
pub fn query_raw(
    target: &ContractId,
    query: &Query,
    gas_limit: u64,
) -> Result<ReturnValue, CanonError> {
    let mut buf = [0u8; BUFFER_SIZE_LIMIT];
    let mut sink = Sink::new(&mut buf);

    query.encode(&mut sink);

    unsafe { external::query(&target.as_bytes()[0], &mut buf[0], gas_limit) }

    // read return back
    let mut source = Source::new(&buf);

    ReturnValue::decode(&mut source)
}

/// Call another contract at address `target`
///
/// Note that you will have to specify the expected return and argument types
/// yourself.
pub fn query<A, R>(
    target: &ContractId,
    query: &A,
    gas_limit: u64,
) -> Result<R, CanonError>
where
    A: Canon,
    R: Canon,
{
    let wrapped = Query::from_canon(query);
    let result = query_raw(target, &wrapped, gas_limit)?;
    result.cast()
}

/// Call another contract at address `target`
pub fn transact_raw<Slf>(
    slf: &mut Slf,
    target: &ContractId,
    transaction: &Transaction,
    gas_limit: u64,
) -> Result<ReturnValue, CanonError>
where
    Slf: Canon,
{
    let mut buf = [0u8; BUFFER_SIZE_LIMIT];
    let mut sink = Sink::new(&mut buf);

    // Store the state before call `transact`
    let state = ContractState::from_canon(slf);

    state.encode(&mut sink);
    transaction.encode(&mut sink);

    unsafe { external::transact(&target.as_bytes()[0], &mut buf[0], gas_limit) }

    // read return back
    let mut source = Source::new(&buf);

    let state = ContractState::decode(&mut source)?;

    *slf = state.cast()?;

    ReturnValue::decode(&mut source)
}

/// Call another contract at address `target`
///
/// Note that you will have to specify the expected return and argument types
/// yourself.
pub fn transact<A, R, Slf>(
    slf: &mut Slf,
    target: &ContractId,
    transaction: &A,
    gas_limit: u64,
) -> Result<R, CanonError>
where
    A: Canon,
    R: Canon,
    Slf: Canon,
{
    let wrapped = Transaction::from_canon(transaction);
    let result = transact_raw(slf, &target, &wrapped, gas_limit)?;

    result.cast()
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate alloc;

    use alloc::vec::Vec;
    use canonical_fuzz::*;
    use canonical_host::MemStore;

    use arbitrary::{Arbitrary, Unstructured};

    impl Arbitrary for Buffer<BUFFER_SIZE_LIMIT> {
        fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Self> {
            let mut vec = Vec::arbitrary(u)?;
            vec.truncate(BUFFER_SIZE_LIMIT);
            Ok(Buffer::from_slice(&vec[..]))
        }
    }

    impl Arbitrary for ReturnValue {
        fn arbitrary(u: &mut Unstructured<'_>) -> arbitrary::Result<Self> {
            Ok(ReturnValue(Buffer::arbitrary(u)?))
        }
    }

    #[test]
    fn fuzz_buffer() {
        let store = MemStore::new();
        fuzz_canon::<Buffer<BUFFER_SIZE_LIMIT>, _>(store.clone());
        fuzz_canon::<ReturnValue, _>(store);
    }
}
