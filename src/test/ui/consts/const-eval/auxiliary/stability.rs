// Crate that exports a const fn. Used for testing cross-crate.

#![crate_type="rlib"]
#![stable(feature = "rust1", since = "1.0.0")]

#![feature(const_fn)]
#![feature(staged_api)]

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature="foo", issue = "0")]
pub const fn foo() -> u32 { 42 }
