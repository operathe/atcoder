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
        mut a: [usize; n],
    }
    a.sort_unstable_by(|a, b| b.cmp(a));
    let mut ans = 0;
    while a[1] > 0 {
        ans += 1;
        a[0] -= 1;
        a[1] -= 1;
        a.sort_unstable_by(|a, b| b.cmp(a));
    }
    println!("{}", ans);
}
