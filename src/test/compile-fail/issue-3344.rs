// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct thing(uint);
impl cmp::Ord for thing { //~ ERROR missing method `gt`
    fn lt(&self, other: &thing) -> bool { **self < **other }
    fn le(&self, other: &thing) -> bool { **self < **other }
    fn ge(&self, other: &thing) -> bool { **self < **other }
}
fn main() {}