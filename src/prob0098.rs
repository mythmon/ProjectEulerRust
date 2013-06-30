#[link(name = "prob0098", vers = "0.0")];
#[crate_type = "lib"];

extern mod extra;
extern mod common;

use std::{util, uint, vec, io};
use std::iterator::OrdIterator;
use std::hashmap::HashMap;
use extra::sort;
use common::{arith, calc, reader};

pub static expected_answer: &'static str = "18769";

#[inline(always)]
fn check_digit(idx: &[uint], ds: &[uint]) -> bool {
    for uint::range(0, idx.len()) |i| {
        if ds[i] != ds[idx[i]] { return false; }
        if ds.position_elem(&ds[idx[i]]).get() != idx[i] { return false; }
    }
    return true;
}

#[inline(always)]
fn idx_to_num(idx: &[uint], ds: &[uint]) -> uint {
    idx.iter().fold(0u, |num, &i| 10 * num + ds[i])
}

#[inline(always)]
fn is_square(n: uint) -> bool {
    let sq = arith::isqrt(n);
    return (sq * sq == n);
}

pub fn solve() -> ~str {
    let result = io::read_whole_file_str(&Path("files/words.txt"))
        .chain(|input| {
            do reader::read_whole_word(input).map |words| {
                let mut map = ~HashMap::new();
                for words.iter().advance |&word| {
                    let mut cs: ~[char] = word.iter().collect();
                    sort::quick_sort(cs, |a, b| a <= b);
                    match map.pop(&cs) {
                        None     => { map.insert(cs, ~[word.to_str()]); }
                        Some(ws) => { map.insert(cs, ws + [word.to_str()]); }
                    }
                }
                map
            }
        }).map(|mut &map| {
            do vec::build |push| {
                for map.mutate_values |_key, values| {
                    if values.len() > 1 {
                        push(util::replace(values, ~[]));
                    }
                }
            }
        }).map(|&words| {
            do vec::build_sized(words.len()) |push| {
                for words.iter().advance |elt| {
                    for uint::range(0, elt.len()) |i| {
                        for uint::range(i + 1, elt.len()) |j| {
                            push((elt[i].clone(), elt[j].clone()))
                        }
                    }
                }
            }
        }).map(|&word_pairs| {
            do word_pairs.map |&(w1, w2)| {
                let cs1 = w1.as_bytes();
                let cs2 = w2.as_bytes();
                let get_pos = |&c: &u8| cs1.position_elem(&c).get();
                (w1.len(), cs1.map(get_pos), cs2.map(get_pos))
            }
        }).map(|mut &words| {
            sort::quick_sort(words, |&(l1, _, _), &(l2, _, _)| l1 >= l2);
            words
        }).map(|&idx_pairs| {
            do vec::build |push| {
                let mut cur_len = uint::max_value;
                let mut cur_group = ~[];
                for idx_pairs.iter().advance |&(len, v1, v2)| {
                    if cur_group.is_empty() || cur_len == len {
                        cur_len = len;
                        cur_group.push((v1, v2));
                    } else {
                        push((cur_len, util::replace(&mut cur_group, ~[(v1, v2)])));
                        cur_len = len;
                    }
                }
                if !cur_group.is_empty() { push((cur_len, cur_group)); }
            }
        }).map(|&groups| {
            let mut max = 0;

            for groups.iter().advance |&(len, pairs)| {
                let mut nums = ~[];

                let start = calc::pow(10, len) - 1;
                let end   = calc::pow(10, len - 1);
                for uint::range_rev(arith::isqrt(start), arith::isqrt(end)) |n| {
                    let ds = calc::num_to_digits(n * n, 10);
                    for pairs.iter().advance |&(v1, v2)| {
                        if ds[v2[0]] == 0 { loop; }
                        if !check_digit(v1, ds) { loop; }
                        let num2 = idx_to_num(v2, ds);
                        if !is_square(num2) { loop; }
                        nums.push(n * n);
                        if n * n != num2 { nums.push(num2); }
                    }
                }

                if !nums.is_empty() {
                    max = nums.iter().transform(|&x| x).max().get();
                    break;
                }
            }
            max
        });

    match result {
        Err(msg) => fail!(msg),
        Ok(value) => return value.to_str()
    }
}