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
    let mut a = vec![vec![]; n];

    for i in 0..n {
        input! {
                v: [Usize1; i + 1],
        }
        a[i] = v;
    }
    let mut now = 0;
    for i in 0..n {
        if now >= i {
            now = a[now][i];
        } else {
            now = a[i][now];
        }
    }
    println!("{}", now + 1);
}
