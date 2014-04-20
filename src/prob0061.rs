#![crate_id = "prob0061"]
#![crate_type = "rlib"]

use std::iter::AdditiveIterator;

pub static EXPECTED_ANSWER: &'static str = "28684";

fn create_map(f: |uint| -> uint) -> Vec<Vec<uint>> {
    let mut result = Vec::from_fn(100, |_| Vec::new());
    let mut i = 1;
    loop {
        let n = f(i);
        if n >= 10000 { break }
        result.get_mut(n / 100).push(n % 100);
        i += 1;
    }
    result
}

pub fn solve() -> ~str {
    let map = [
        create_map(|n| n * (n + 1) / 2),
        create_map(|n| n * n),
        create_map(|n| n * (3 * n - 1) / 2),
        create_map(|n| n * (2 * n - 1)),
        create_map(|n| n * (5 * n - 3) / 2),
        create_map(|n| n * (3 * n - 2))
    ];

    let mut result = Vec::new();
    for idx in [0u, 1u, 2u, 3u, 4u].permutations() {
        for (i, v5) in map[5].iter().enumerate() {
            if i < 10 { continue }
            for &n5 in v5.iter() {
                if n5 < 10 { continue }
                for &n0 in map[idx[0]].get(n5).iter() {
                    if n0 < 10 { continue }
                    for &n1 in map[idx[1]].get(n0).iter() {
                        if n1 < 10 { continue }
                        for &n2 in map[idx[2]].get(n1).iter() {
                            if n2 < 10 { continue }
                            for &n3 in map[idx[3]].get(n2).iter() {
                                if n3 < 10 { continue }
                                for &n4 in map[idx[4]].get(n3).iter() {
                                    if n4 < 10 { continue }
                                    if n4 == i {
                                        result.push(~[
                                            n5 * 100 + n0,
                                            n0 * 100 + n1,
                                            n1 * 100 + n2,
                                            n2 * 100 + n3,
                                            n3 * 100 + n4,
                                            n4 * 100 + n5
                                        ]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let sum = result.move_iter().flat_map(|vs| vs.move_iter()).sum();
    sum.to_str()
}

