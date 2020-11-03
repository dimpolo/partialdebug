# partialdebug

Derive Debug for types where not all fields implement Debug.

This relies on specialization and thus requires nightly.

#### Non Exhaustive

Requires the `debug_non_exhaustive` feature.
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
