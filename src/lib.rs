//! # Non Exhaustive
//!
//! ```
//! #![feature(debug_non_exhaustive)]
//! use partialdebug::non_exhaustive::PartialDebug;
//!
//! # #[allow(dead_code)]
//! # struct DNA {
//! #     sequence: &'static str,
//! # }
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
//! #             dna: DNA {
//! #                 sequence: "",
//! #             },
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
//! # #[allow(dead_code)]
//! # struct DNA {
//! #     sequence: &'static str,
//! # }
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
//! #             dna: DNA {
//! #                 sequence: "",
//! #             },
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
//! # #[allow(dead_code)]
//! # struct DNA {
//! #     sequence: &'static str,
//! # }
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
//! #             dna: DNA {
//! #                 sequence: "",
//! #             },
//! #         }
//! #     }
//! # }
//! #
//! assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, dna: Unknown }");
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

pub mod placeholder {
    pub use partialdebug_derive::PlaceholderPartialDebug as PartialDebug;
}
