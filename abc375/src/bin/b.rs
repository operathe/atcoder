#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use libm::sqrt;
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
        mut point: [(i64, i64); n]
    }
    point.push((0, 0));
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut ans = 0.0;
    for (x, y) in point {
        ans += (((prev_x - x).pow(2) + (prev_y - y).pow(2)) as f64).sqrt();
        prev_x = x;
        prev_y = y;
    }
    println!("{}", ans);
}
