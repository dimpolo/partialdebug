#![no_implicit_prelude]
#![no_std]
#![feature(debug_non_exhaustive)]
use ::partialdebug;

// These are required for now
use ::core::option::Option::*;

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
