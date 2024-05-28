// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing terms.
// SPDX-License-Identifier: Apache-2.0 OR MIT indicates dual licensing under Apache 2.0 or MIT licenses.
// Copyright © 2024 LibYML. All rights reserved.

use core::ops::Deref;

pub(crate) const OK: Success = Success { ok: true };
pub(crate) const FAIL: Success = Success { ok: false };

#[must_use]
#[derive(Debug)]
pub struct Success {
    pub ok: bool,
}

#[derive(Debug)]
pub struct Failure {
    pub fail: bool,
}

impl Deref for Success {
    type Target = Failure;

    fn deref(&self) -> &Self::Target {
        if self.ok {
            &Failure { fail: false }
        } else {
            &Failure { fail: true }
        }
    }
}
