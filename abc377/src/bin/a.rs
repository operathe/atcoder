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
        n: usize,
    }
    let mut dp = vec![Mint::new(0); n + 1];
    dp[0] = Mint::new(1);
    for i in 1..=n {
        for j in 1..=6 {
            if i >= j {
                dp[i] += dp[i - j];
            }
        }
    }
    println!("{}", dp[n]);
}
