#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
    }
    let result = is_in_time_range(a, b, c);
    println!("{}", if result { "Yes" } else { "No" });
}

fn is_in_time_range(a: isize, b: isize, c: isize) -> bool {
    if b < c {
        a < b || c < a
    } else {
        a < b && c < a
    }
}
