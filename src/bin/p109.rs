//! [Problem 109](https://projecteuler.net/problem=109) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(core)]

#[macro_use(problem)] extern crate common;
extern crate polynomial;

use std::iter;
use polynomial::Polynomial;

fn count_way(score: u32) -> u32 {
    let mut single = iter::repeat(0u32).take(26).collect::<Vec<_>>();
    let mut double = iter::repeat(0u32).take(51).collect::<Vec<_>>();
    let mut triple = iter::repeat(0u32).take(61).collect::<Vec<_>>();
    let mut dup    = iter::repeat(0u32).take(121).collect::<Vec<_>>();
    for i in (1us .. 21) {
        single[1 * i] = 1;
        double[2 * i] = 1;
        triple[3 * i] = 1;
        dup[2 * i] += 1;
        dup[4 * i] += 1;
        dup[6 * i] += 1;
    }
    single[25] = 1;
    double[50] = 1;
    dup[50]    += 1;
    dup[100]   += 1;

    let single = Polynomial::new(single);
    let double = Polynomial::new(double);
    let triple = Polynomial::new(triple);
    let dup    = Polynomial::new(dup);

    let p_all = &single + &double + &triple;
    let p1    = double.clone();
    let p2    = &double * &p_all;
    let p3    = &double * Polynomial::new((&p_all * &p_all + &dup)
                                          .data()
                                          .iter()
                                          .map(|&n| n / 2)
                                          .collect());
    let total = p1 + p2 + p3;
    total.data().iter().take(score as usize).fold(0, |i, &a| i + a)
}

fn solve() -> String {
    count_way(100).to_string()
}

problem!("38182", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn example() {
        assert_eq!(11, super::count_way(6));
        assert_eq!(42336, super::count_way(171));
    }
}
