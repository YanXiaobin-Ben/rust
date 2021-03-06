// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// ignore-wasm32

#![feature(decl_macro, macros_in_extern)]

macro_rules! returns_isize(
    ($ident:ident) => (
        fn $ident() -> isize;
    )
);

macro takes_u32_returns_u32($ident:ident) {
    fn $ident (arg: u32) -> u32;
}

macro_rules! emits_nothing(
    () => ()
);

fn main() {
    assert_eq!(unsafe { rust_get_test_int() }, 1isize);
    assert_eq!(unsafe { rust_dbg_extern_identity_u32(0xDEADBEEF) }, 0xDEADBEEFu32);
}

#[link(name = "rust_test_helpers", kind = "static")]
extern {
    returns_isize!(rust_get_test_int);
    takes_u32_returns_u32!(rust_dbg_extern_identity_u32);
    emits_nothing!();
}
