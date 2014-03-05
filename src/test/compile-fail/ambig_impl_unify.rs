// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait foo {
    fn foo(&self) -> int;
}

impl foo for Vec<uint> {
    fn foo(&self) -> int {1} //~ NOTE candidate #1 is `~[uint].foo::foo`
}

impl foo for Vec<int> {
    fn foo(&self) -> int {2} //~ NOTE candidate #2 is `~[int].foo::foo`
}

fn main() {
    let x = Vec::new();
    x.foo(); //~ ERROR multiple applicable methods in scope
}
