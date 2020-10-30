//! Derive Debug for types where not all fields implement Debug
//!
//! # Non Exhaustive
//!
//! ```
//! #![feature(debug_non_exhaustive)]
//! use partialdebug::non_exhaustive::PartialDebug;
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
//! assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, .. }");
//! ```
//!
//! # Placeholder with Type Info
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
//! # Placeholder with Custom Text
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

#![allow(incomplete_features)]
#![feature(specialization)]
#![warn(missing_docs, trivial_casts, rust_2018_idioms)]

use core::fmt::{Debug, Formatter, Result};

/// Specialized trait used to distinguish between types that implement Debug and one's that don't^.
/// ```
/// # use partialdebug::AsDebug;
/// # struct DNA;
/// # let dna = DNA;
/// assert!(42.as_debug().is_some());
/// assert!(dna.as_debug().is_none());
/// ```
pub trait AsDebug {
    /// Try to get a reference to `self` as `dyn Debug`
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

/// The non exhaustive version of `PartialDebug`
pub mod non_exhaustive {
    pub use partialdebug_derive::NonExhaustivePartialDebug as PartialDebug;
}

/// The placeholder version of `PartialDebug`
pub mod placeholder {
    pub use partialdebug_derive::PlaceholderPartialDebug as PartialDebug;
}
