#![no_implicit_prelude]
#![no_std]
#![allow(dead_code)]

use ::partialdebug;

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
