#![allow(incomplete_features)]
#![feature(specialization)]

use core::fmt::Debug;

pub trait AsDebug {
    fn as_debug(&self) -> Option<&dyn Debug>;
}

impl<T> AsDebug for T {
    default fn as_debug(&self) -> Option<&dyn Debug> {
        None
    }
}

impl<T: Debug> AsDebug for T {
    fn as_debug(&self) -> Option<&dyn Debug> {
        Some(self)
    }
}

pub mod non_exhaustive {
    pub use partialdebug_derive::NonExhaustivePartialDebug as PartialDebug;
}
