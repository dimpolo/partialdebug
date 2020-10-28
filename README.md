# partialdebug

## Non Exhaustive

```rust
#![feature(debug_non_exhaustive)]

use partialdebug::non_exhaustive::PartialDebug;

#[allow(dead_code)]
struct DNA {
    sequence: &'static str,
}

#[derive(PartialDebug)]
struct Dog {
    legs: usize,
    eyes: usize,
    dna: DNA,
}

let dog = Dog {
    legs: 4,
    eyes: 2,
    dna: DNA {
        sequence: "GACCCCGATTTGA",
    },
};
assert_eq!(format!("{:?}", dog), "Dog { legs: 4, eyes: 2, .. }");
```

## Type Info

```rust
use partialdebug::type_info::PartialDebug;

#[allow(dead_code)]
struct DNA {
    sequence: &'static str,
}

#[derive(PartialDebug)]
struct Dog {
    legs: usize,
    eyes: usize,
    dna: DNA,
}

let dog = Dog {
    legs: 4,
    eyes: 2,
    dna: DNA {
        sequence: "GACCCCGATTTGA",
    },
};
assert_eq!(format!("{:?}", dog), "Dog { legs: 4, eyes: 2, dna: DNA }");
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
