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
        M: usize,
        D: usize,
        y: usize,
        m: usize,
        d: usize,
    }
    if m == M && d == D {
        println!("{} {} {}", y + 1, 1, 1);
    } else if d == D {
        println!("{} {} {}", y, m + 1, 1);
    } else {
        println!("{} {} {}", y, m, d + 1);
    }
}
