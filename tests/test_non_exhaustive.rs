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

#[test]
fn test() {
    let dog = Dog {
        legs: 4,
        eyes: 2,
        dna: DNA {
            sequence: "GACCCCGATTTGA",
        },
    };
    assert_eq!(format!("{:?}", dog), "Dog { legs: 4, eyes: 2, .. }");
}
