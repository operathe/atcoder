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
        s: u32,
        m: u32,
        l: u32
    }
    // let mut ans = usize::MAX;
    // for i in 0..=100 {
    //     for j in 0..=100 {
    //         for k in 0..=100 {
    //             if i * 6 + j * 8 + k * 10 >= n {
    //                 ans = min(ans, i * s + j * m + k * l);
    //             }
    //         }
    //     }
    // }
    // println!("{}", ans);
    let mut dp = vec![u32::MAX; n + 13];
    dp[0] = 0;
    for i in 0..n {
        let x = dp[i];
        if x == u32::MAX {
            continue;
        }
        dp[i + 6] = min(dp[i + 6], x + s);
        dp[i + 8] = min(dp[i + 8], x + m);
        dp[i + 12] = min(dp[i + 12], x + l);
    }
    let ans = dp[n..].iter().min().unwrap();
    println!("{}", ans);
}
