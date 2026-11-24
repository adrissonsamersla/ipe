//! Implement useful leaves that force a specific status { Idle, Running, Success, Failure, Error }.

use crate::{Leaf, Status};

pub struct ForceIdle {}
impl<W, S, A, B> Leaf<W, S, A, B> for ForceIdle {
    const KIND: &'static str = "force_idle";
    type Config = ();
    fn tick(_: &(), _: &W, _: &S, _: &mut A, _: &mut B) -> Status {
        Status::Idle
    }
}

pub struct ForceRunnig {}
impl<W, S, A, B> Leaf<W, S, A, B> for ForceRunnig {
    const KIND: &'static str = "force_running";
    type Config = ();
    fn tick(_: &(), _: &W, _: &S, _: &mut A, _: &mut B) -> Status {
        Status::Running
    }
}

pub struct ForceSuccess {}
impl<W, S, A, B> Leaf<W, S, A, B> for ForceSuccess {
    const KIND: &'static str = "force_success";
    type Config = ();
    fn tick(_: &(), _: &W, _: &S, _: &mut A, _: &mut B) -> Status {
        Status::Success
    }
}

pub struct ForceFailure {}
impl<W, S, A, B> Leaf<W, S, A, B> for ForceFailure {
    const KIND: &'static str = "force_failure";
    type Config = ();
    fn tick(_: &(), _: &W, _: &S, _: &mut A, _: &mut B) -> Status {
        Status::Failure
    }
}

pub struct ForceError {}
impl<W, S, A, B> Leaf<W, S, A, B> for ForceError {
    const KIND: &'static str = "force_error";
    type Config = ();
    fn tick(_: &(), _: &W, _: &S, _: &mut A, _: &mut B) -> Status {
        Status::Error
    }
}
