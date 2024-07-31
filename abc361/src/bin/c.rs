#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::pow;
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize, k: usize,
        mut a: [usize; n],
    }
    let m = n - k;
    a.sort();
    let ans = (0..=k).map(|i| a[i + m - 1] - a[i]).min().unwrap();
    println!("{}", ans);
}
