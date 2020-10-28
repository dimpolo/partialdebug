//! # Non Exhaustive
//!
//! ```
//! #![feature(debug_non_exhaustive)]
//!
//! use partialdebug::non_exhaustive::PartialDebug;
//!
//! #[allow(dead_code)]
//! struct DNA {
//!     sequence: &'static str,
//! }
//!
//! #[derive(PartialDebug)]
//! struct Dog {
//!     legs: usize,
//!     eyes: usize,
//!     dna: DNA,
//! }
//!
//! let dog = Dog {
//!     legs: 4,
//!     eyes: 2,
//!     dna: DNA {
//!         sequence: "GACCCCGATTTGA",
//!     },
//! };
//! assert_eq!(format!("{:?}", dog), "Dog { legs: 4, eyes: 2, .. }");
//! ```
//!
//! # Type Info
//!
//! ```
//! #![feature(debug_non_exhaustive)]
//!
//! use partialdebug::type_info::PartialDebug;
//!
//! #[allow(dead_code)]
//! struct DNA {
//!     sequence: &'static str,
//! }
//!
//! #[derive(PartialDebug)]
//! struct Dog {
//!     legs: usize,
//!     eyes: usize,
//!     dna: DNA,
//! }
//!
//! let dog = Dog {
//!     legs: 4,
//!     eyes: 2,
//!     dna: DNA {
//!         sequence: "GACCCCGATTTGA",
//!     },
//! };
//! assert_eq!(format!("{:?}", dog), "Dog { legs: 4, eyes: 2, dna: DNA }");
//! ```

#![allow(incomplete_features)]
#![feature(specialization)]

use core::fmt::{Debug, Formatter, Result};

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

pub struct Placeholder(pub &'static str);

impl Debug for Placeholder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple(self.0).finish()
    }
}

pub mod non_exhaustive {
    pub use partialdebug_derive::NonExhaustivePartialDebug as PartialDebug;
}

pub mod type_info {
    pub use partialdebug_derive::TypeInfoPartialDebug as PartialDebug;
}
