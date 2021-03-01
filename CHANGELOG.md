# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2021-03-01

### Added

- Add `cast` method to `ContractState`

### Changed

- Change `transact_raw` to returns `(ContractState, ReturnValue)` instead of `ReturnValue`
- Change `transact` to accept a third generic `Slf` representing the Contract's Self

## [0.5.1] - 2021-02-25

### Changed

- Change buffer limit from 1KB to 2KB for WASM transaction

## [0.5.0] - 2021-02-18

### Added

- Add Module trait to resolve the module's id

### Changed

- Change `HostModule` trait to be a subtrait of `Module`

## [0.4.0] - 2021-02-17

### Added

- Add CHANGELOG.md
- Add README.md
- Add Github Actions
- Add Poseidon hash as external host function
- Add Initial ABI structure for external host functions
- Add backend and tests for Proof verification
- Add ProofVerification ABICall mock
- Add `block_height` as host function
- Add Poseidon Sponge Hash function as Host function
- Add `WeeAlloc` in contracts
- Add proper license header

### Changed

- Change folder structure of `dusk-abi` modules
- Change ABI methods' names
- Change `contract_id` to be taken as value

### Removed

- Remove `verify_proof` as host function from rusk-vm
- Remove Poseidon Hash from Rusk VM
- Remove `debug!` call from `verify_proof`
- Remove the `include!()` macro for panic handling

### Fixed

- Fix the code's formatting with cargo fmt

## [0.3.0] - 2021-01-19

### Added

- Add Canonical stack to `dusk-abi`
- Add LICENSE

### Changed

- Use canonical published library versions from crates.io

## [0.2.0] - 2020-04-20

### Added

- Add Storage ABI and Test Contract
- Add documentation
- Add `from_bytes` method for Provisioners
- Add `to_bytes` to Provisioners
- Add closing parentheses in provisioners debug print
- Add proofs to Transfer and Approve calls
- Add missing types and methods to StakingCall enum
- Add re-exporting for `Transfer` and `Staking` contract call enums
- Add fee contract call enum to `dusk-abi`
- Add a type for Provisioners addresses
- Add a host function for crediting Phoenix notes
- Add host functions for BLS signature verification and phoenix note checks
- Add documentation to public interface
- Add test to ensure implementation of Serialize/Deserialize is correct
- Add a macro which implements Serialize and Deserialize
- Add `Content` implementation for `NetworkState`
- Add Kelvin integration
- Add gas host function
- Add gas test contract
- Add Recursive contract calls and return values
- Add the capability to store arbitrary (de)serializable values in contract state
- Add Contract signature verification
- Add DefaultAccont contract interface
- Add integration with `fermion`
- Add contract calls capability
- Add test to check a deployed contract initial state
- Add Contract storage

### Changed

- Change git imports to use https
- Change calling conventions
- Change `phoenix` and `phoenix-abi` deps to master branch
- Change the usage of phoenix ABI nullifiers for phoenix ABI inputs
- Update phoenix-abi imports
- Change Provisioners internals visibility from public to private
- Change contract call enums, moved out of contract files
- Change public keys to be 64 bytes
- Change the serde array macro to be no_std compatible
- Change serde implementation macro to be in its own file
- Rename macro to denote its target type
- Change error message to based on array length
- Change macro so that it can impl Serialize/Deserialize for any length of array, not just 64
- Update to kelvin 0.9

### Removed

- Remove serde dependencies
- Remove warnings
- Remove leftover code for the staking contract
- Remove obsolete host functions
- Remove SignatureSerializationHack in favor of using the impl_serde macro
- Remove obsolete `genesis` crate
- Remove mutability `ContractCall` argument for `call_contract`

### Fixed

- Fix decoding of transaction
- Fix warning and typos

## [0.1.0] - 2019-08-12

### Added

- Basic transaction framework

[unreleased]: https://github.com/dusk-network/dusk-abi/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/dusk-network/dusk-abi/compare/v0.5.1...v0.6.0
[0.5.1]: https://github.com/dusk-network/dusk-abi/compare/v0.5.0...v0.5.1
[0.5.0]: https://github.com/dusk-network/dusk-abi/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/dusk-network/dusk-abi/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/dusk-network/dusk-abi/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/dusk-network/dusk-abi/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/dusk-network/dusk-abi/releases/tag/v0.1.0
