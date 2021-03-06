// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct UnitStruct;
struct TupleStruct();
struct BracedStruct{}

enum E {
    UnitVariant,
    TupleVariant(),
    BracedVariant{},
}
use E::*;

const CONST: () = ();
static STATIC: () = ();

fn function() {}

fn main() {
    let doesnt_matter = 0;

    match UnitStruct {
        UnitStruct => {} // OK, `UnitStruct` is a unit struct pattern
    }
    match doesnt_matter {
        TupleStruct => {} //~ ERROR match bindings cannot shadow tuple structs
    }
    match doesnt_matter {
        BracedStruct => {} // OK, `BracedStruct` is a fresh binding
    }
    match UnitVariant {
        UnitVariant => {} // OK, `UnitVariant` is a unit variant pattern
    }
    match doesnt_matter {
        TupleVariant => {} //~ ERROR match bindings cannot shadow tuple variants
    }
    match doesnt_matter {
        BracedVariant => {} //~ ERROR match bindings cannot shadow struct variants
    }
    match CONST {
        CONST => {} // OK, `CONST` is a const pattern
    }
    match doesnt_matter {
        STATIC => {} //~ ERROR match bindings cannot shadow statics
    }
    match doesnt_matter {
        function => {} // OK, `function` is a fresh binding
    }

    let UnitStruct = UnitStruct; // OK, `UnitStruct` is a unit struct pattern
    let TupleStruct = doesnt_matter; //~ ERROR let bindings cannot shadow tuple structs
    let BracedStruct = doesnt_matter; // OK, `BracedStruct` is a fresh binding
    let UnitVariant = UnitVariant; // OK, `UnitVariant` is a unit variant pattern
    let TupleVariant = doesnt_matter; //~ ERROR let bindings cannot shadow tuple variants
    let BracedVariant = doesnt_matter; //~ ERROR let bindings cannot shadow struct variants
    let CONST = CONST; // OK, `CONST` is a const pattern
    let STATIC = doesnt_matter; //~ ERROR let bindings cannot shadow statics
    let function = doesnt_matter; // OK, `function` is a fresh binding
}
