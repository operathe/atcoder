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
        wx: [(isize, usize); n]
    }
    let m = 24;
    let mut count = vec![0; m];
    for (w, x) in wx {
        for j in 9..18 {
            count[(j + x) % m] += w;
        }
    }
    println!("{}", count.iter().max().unwrap());
}
