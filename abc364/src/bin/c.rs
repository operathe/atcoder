#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min, Reverse};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        xlim: u64,
        ylim: u64,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    a.sort_by_key(|&x| Reverse(x));
    b.sort_by_key(|&x| Reverse(x));
    let mut ans = 0;
    let mut x = 0;
    let mut y = 0;
    for (&dx, &dy) in a.iter().zip(&b) {
        x += dx;
        y += dy;
        ans += 1;
        if xlim < x || ylim < y {
            break;
        }
    }
    println!("{}", ans);
}
