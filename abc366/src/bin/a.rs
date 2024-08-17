#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    non_snake_case,
    unreachable_code
)]
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
        n: usize,
        t: usize,
        a: usize,
    }
    let result = match (t > (n / 2), a > (n / 2)) {
        (true, _) => "Yes",
        (_, true) => "Yes",
        _ => "No",
    };
    println!("{}", result);
}
