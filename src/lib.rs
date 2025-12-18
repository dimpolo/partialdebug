//! Derive Debug for types where not all fields implement Debug.
//!
//! This crate works on stable and with `no_std`.
//! On nightly the `unstable` feature can be used for specialization-based trait detection.
//!
//! ### Placeholder with Type Info
//!
//! ```
//! use partialdebug::placeholder::PartialDebug;
//!
//! # struct DNA;
//! #
//! #[derive(PartialDebug)]
//! struct Dog {
//!     legs: usize,
//!     eyes: usize,
//!     dna: DNA,
//! }
//!
//! # impl Dog {
//! #     fn new() -> Dog {
//! #         Dog {
//! #             legs: 4,
//! #             eyes: 2,
//! #             dna: DNA,
//! #         }
//! #     }
//! # }
//! #
//! assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, dna: DNA }");
//! ```
//!
//! ### Placeholder with Custom Text
//!
//! ```
//! use partialdebug::placeholder::PartialDebug;
//!
//! # struct DNA;
//! #
//! #[derive(PartialDebug)]
//! #[debug_placeholder = "Unknown"]
//! struct Dog {
//!     legs: usize,
//!     eyes: usize,
//!     dna: DNA,
//! }
//!
//! # impl Dog {
//! #     fn new() -> Dog {
//! #         Dog {
//! #             legs: 4,
//! #             eyes: 2,
//! #             dna: DNA,
//! #         }
//! #     }
//! # }
//! #
//! assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, dna: Unknown }");
//! ```
//!
//! ### Non-Exhaustive
//!
//! Only available for structs with named fields.
//!
//! ```
//! use partialdebug::non_exhaustive::PartialDebug;
//! # struct DNA;
//! #
//! #[derive(PartialDebug)]
//! struct Dog {
//!     legs: usize,
//!     eyes: usize,
//!     dna: DNA,
//! }
//! # impl Dog {
//! #     fn new() -> Dog {
//! #         Dog {
//! #             legs: 4,
//! #             eyes: 2,
//! #             dna: DNA,
//! #         }
//! #     }
//! # }
//! #
//! assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, .. }");
//! ```
//!
//! ### Caveats
//!
//! Trait detection for generic types requires specialization.
//! To enable specialization based trait detection, use a nightly compiler and enable the `unstable` feature.
//!
//! ```
//! use partialdebug::placeholder::PartialDebug;
//!
//! #[derive(PartialDebug)]
//! struct Container<T>(T);
//!
//! #[cfg(feature = "unstable")]
//! assert_eq!(format!("{:?}", Container(42)), "Container(42)");
//! #[cfg(not(feature = "unstable"))]
//! assert_eq!(format!("{:?}", Container(42)), "Container(T)");
//! ```

#![no_std]
#![warn(missing_docs, trivial_casts, rust_2018_idioms)]
#![cfg_attr(feature = "unstable", feature(try_as_dyn))]

use core::fmt::{Debug, Formatter, Result};

/// Placeholder struct for types that do not implement Debug
/// ```
/// # use partialdebug::Placeholder;
/// assert_eq!(format!("{:?}", Placeholder("Foo")), "Foo")
/// ```
pub struct Placeholder(pub &'static str);

impl Debug for Placeholder {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple(self.0).finish()
    }
}

#[cfg(any(not(feature = "unstable"), doc))]
pub mod no_specialization;

/// The placeholder version of `PartialDebug`
pub mod placeholder {
    #[cfg(not(feature = "unstable"))]
    pub use crate::no_specialization::NotDebug as PartialDebug; // needs to be in scope
    pub use partialdebug_derive::PlaceholderPartialDebug as PartialDebug;
}

/// The non exhaustive version of `PartialDebug`
pub mod non_exhaustive {
    #[cfg(not(feature = "unstable"))]
    pub use crate::no_specialization::NotDebug as PartialDebug; // needs to be in scope
    pub use partialdebug_derive::NonExhaustivePartialDebug as PartialDebug;
}
