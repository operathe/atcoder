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
    let mut l = 0;
    let mut r = 1_000_000_000_000_001;
    while r - l > 1 {
        let x = (l + r) / 2;
        let mut sum = 0;
        for &a in &a {
            sum += min(a, x);
        }
        if sum <= m {
            l = x;
        } else {
            r = x;
        }
    }
    if l == 1_000_000_000_000_000 {
        println!("infinite");
    } else {
        println!("{}", l);
    }
}
