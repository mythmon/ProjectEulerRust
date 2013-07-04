#[link(name = "prob0063", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;


use std::uint;
use extra::bigint::{BigUint};

pub static EXPECTED_ANSWER: &'static str = "49";

pub fn solve() -> ~str {
    let mut cnt = 1u; // a == 1
    for uint::range(2, 10) |a_uint| {
        let a = BigUint::from_uint(a_uint);
        let mut n = 0;
        let mut an = BigUint::from_uint(1);
        loop {
            n += 1;
            an = an * a;
            let an_str = an.to_str();
            if an_str.len() != n { break; }

            cnt += 1;
        }
    }

    return cnt.to_str();
}

