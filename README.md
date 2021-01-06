# partialdebug
[<img alt="github" src="https://img.shields.io/badge/github-grey?style=for-the-badge&labelColor=555555&logo=github">](https://github.com/dimpolo/partialdebug)
[<img alt="Crates.io" src="https://img.shields.io/crates/v/partialdebug?logo=rust&style=for-the-badge">](https://crates.io/crates/partialdebug)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-partialdebug-blue?style=for-the-badge&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K">](https://docs.rs/partialdebug)

Derive Debug for types where not all fields implement Debug.

This crate works on stable and with `no_std`.
On nightly the `unstable` feature can be used for specialization based trait detection and/or `..` formatting.

#### Placeholder with Type Info

```rust
use partialdebug::placeholder::PartialDebug;

#[derive(PartialDebug)]
struct Dog {
    legs: usize,
    eyes: usize,
    dna: DNA,
}

assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, dna: DNA }");
```

#### Placeholder with Custom Text

```rust
use partialdebug::placeholder::PartialDebug;

#[derive(PartialDebug)]
#[debug_placeholder = "Unknown"]
struct Dog {
    legs: usize,
    eyes: usize,
    dna: DNA,
}

assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, dna: Unknown }");
```

#### Non Exhaustive

Only available on nightly after setting the `unstable` feature.

Requires the `debug_non_exhaustive` feature to be enabled in user code.

Only available for structs with named fields.

```rust
#![feature(debug_non_exhaustive)]
use partialdebug::non_exhaustive::PartialDebug;

#[derive(PartialDebug)]
struct Dog {
    legs: usize,
    eyes: usize,
    dna: DNA,
}

assert_eq!(format!("{:?}", Dog::new()), "Dog { legs: 4, eyes: 2, .. }");
```

#### Caveats

Trait detection for generic types requires specialization.
To enable specialization based trait detection use a nightly compiler and enable the `unstable` feature.

```rust
use partialdebug::placeholder::PartialDebug;

#[derive(PartialDebug)]
struct Container<T>(T);

#[cfg(feature = "unstable")]
assert_eq!(format!("{:?}", Container(42)), "Container(42)");
#[cfg(not(feature = "unstable"))]
assert_eq!(format!("{:?}", Container(42)), "Container(T)");
```
#### License
<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
