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
        y: usize,
    }

    let mut ans = if y % 400 == 0 {
        366
    } else if y % 100 == 0 && y % 400 != 0 {
        365
    } else if y % 4 != 0 {
        365
    } else if y % 4 == 0 && y % 100 != 0 {
        366
    } else {
        365
    };
    println!("{}", ans);
}
