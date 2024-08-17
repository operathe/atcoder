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
        m: usize,
        a: [usize; n],
    }

    let result = count_pairs(n, m, a);
    println!("{}", result);
}

fn count_pairs(n: usize, m: usize, a: Vec<usize>) -> usize {
    let mut cumsum = vec![0; 2 * n + 1];
    for i in 0..2 * n {
        cumsum[i + 1] = (cumsum[i] + a[i % n]) % m;
    }

    let mut count = vec![0; m];
    let mut result = 0;

    for i in 0..n {
        count[cumsum[i]] += 1;
    }

    for i in 0..n {
        result += count[cumsum[i + n]];
        if cumsum[i + n] == cumsum[i] {
            result -= 1; // 自分自身とのペアを除外
        }
        count[cumsum[i]] -= 1;
        count[cumsum[i + n]] += 1;
    }

    result
}
