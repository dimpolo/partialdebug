//! Trait detection logic without using specialization.
//!
//! Inspired by the [impls](https://github.com/nvzqz/impls#how-it-works) crate.
//!
//! [`DebugDetector`] implements the method `as_debug` twice.
//! Once as a conditionally implemented inherent method always returning `Some(&dyn Debug)` and once as a trait method always returning `None`.
//!
//! An inherent method has priority over a trait method of the same name.
//!
//! Using `DebugDetector::<SomeType>::as_debug(&something)` then acts as a trait detector.
//!
//! Note that if `SomeType` is generic the detector won't work.

use core::fmt::Debug;

/// Fallback trait implemented by DebugDetector.
pub trait NotDebug: private::Sealed {
    /// Returns `None`
    #[allow(clippy::wrong_self_convention)]
    fn as_debug<T>(_: &T) -> Option<&dyn Debug> {
        None
    }
}

impl<T> NotDebug for DebugDetector<T> {}

/// Zero size struct, acting as a trait Detector.
pub struct DebugDetector<T>(core::marker::PhantomData<T>);

impl<T: Debug> DebugDetector<T> {
    /// Returns the passed in argument as `Some(&dyn Debug).`
    #[allow(clippy::unnecessary_wraps, clippy::wrong_self_convention)]
    pub fn as_debug(it: &T) -> Option<&dyn Debug> {
        Some(it)
    }
}

mod private {
    pub trait Sealed {}

    // Implement for those same types, but no others.
    impl<T> Sealed for super::DebugDetector<T> {}
}
