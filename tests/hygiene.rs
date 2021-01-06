#![no_implicit_prelude]
#![no_std]
#![cfg_attr(feature = "unstable", feature(debug_non_exhaustive))]

use ::partialdebug;

// These are required for now
use ::core::option::Option::*;

#[cfg_attr(
    feature = "unstable",
    derive(partialdebug::non_exhaustive::PartialDebug)
)]
#[cfg(feature = "unstable")]
struct NonExhaustive {
    field: usize,
}

#[derive(partialdebug::placeholder::PartialDebug)]
struct DefaultPlaceholder {
    field: usize,
}

#[derive(partialdebug::placeholder::PartialDebug)]
#[debug_placeholder = "Unknown"]
struct CustomPlaceholder {
    field: usize,
}
