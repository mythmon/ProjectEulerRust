#![warn(unused, bad_style,
        unused_qualifications, unused_typecasts, unused_results)]

extern crate common;
extern crate prime;

use std::iter::AdditiveIterator;
use common::Solver;
use prime::{PrimeSet, Factorize};

fn compute(limit: uint) -> uint {
    let ps = PrimeSet::new();

    let sum_of_div = Vec::from_fn(limit, |n| n.sum_of_proper_divisor(&ps));

    sum_of_div
        .iter()
        .map(|&n| n)
        .enumerate()
        .filter(|&(n, div)| div < n)
        .filter(|&(n, div)| sum_of_div[div] == n)
        .map(|(a, b)| a + b)
        .sum()
}

fn solve() -> String { compute(10000).to_string() }

fn main() { Solver::new("31626", solve).run(); }