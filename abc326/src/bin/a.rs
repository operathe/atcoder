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
        x: isize,
        y: isize,
    }
    let ans = if (-3..=2).contains(&(y - x)) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
