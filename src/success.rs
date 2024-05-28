use core::ops::Deref;

pub(crate) const OK: Success = Success { ok: true };
pub(crate) const FAIL: Success = Success { ok: false };

#[must_use]
#[derive(Copy, Clone, Debug)]
pub struct Success {
    pub ok: bool,
}

#[derive(Copy, Clone, Debug)]
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
