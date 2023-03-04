#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// To use the `unsafe` keyword, do not remove the `unsafe_code` attribute entirely.
// Instead, change it to `#![deny(unsafe_code)]` + opt-in with locally-justified in comments
// `#[allow(unsafe_code)]`'s on a case-by-case basis.
#![deny(unsafe_code)]
// Safety-critical application lints
#![forbid(bare_trait_objects, overflowing_literals)]
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::semicolon_if_nothing_returned,
    clippy::wildcard_imports
)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
// #![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
// #![allow(clippy::blanket_clippy_restriction_lints, clippy::implicit_return)]

#![no_std]

