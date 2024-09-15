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
        a: [usize; n],
    }
    //長さnの数列aが与えられる
    //f(l,r) = (a[l], a[l+1], ..., a[r])に含まれる値の種類数とする
    // i=1∑Nj=i∑Nf(i,j)を求める
    // 1 <= n <= 2*10^5
    // 1 <= a[i] <= n
    let mut last_seen = vec![0; n + 1];
    let mut dp = vec![0; n + 1];
    let mut total = 0;
    let mut current = 0;

    for i in 1..=n {
        let num = a[i - 1];
        current += i - last_seen[num];
        dp[i] = current;
        total += current;
        last_seen[num] = i;
    }

    println!("{}", total);
}
