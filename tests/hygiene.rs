#![no_implicit_prelude]
#![no_std]
#![cfg_attr(feature = "unstable", feature(debug_non_exhaustive))]
#![allow(dead_code)]

use ::partialdebug;

#[cfg(feature = "unstable")]
#[derive(partialdebug::non_exhaustive::PartialDebug)]
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
