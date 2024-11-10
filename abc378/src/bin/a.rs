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
        a: [Usize1; 4],
    }
    let mut ans = 0;
    let mut count = vec![0usize; 4];
    for &x in a.iter() {
        count[x] += 1;
    }
    for i in 0..4 {
        ans += count[i] / 2;
    }
    println!("{}", ans);
}
