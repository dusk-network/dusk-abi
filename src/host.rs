// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::{Module, Query, ReturnValue};
use canonical::CanonError;

/// The trait that host function modules use to communicate with the VM
pub trait HostModule: Module {
    /// Execute a query for the current module
    fn execute(&self, query: Query) -> Result<ReturnValue, CanonError>;
}
