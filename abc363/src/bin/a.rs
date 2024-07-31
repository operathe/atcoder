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
        r: usize,
    }
    let mut ans = 0;
    if r < 100 {
        ans = 100 - r;
    } else if 100 <= r && r < 200 {
        ans = 200 - r;
    } else if 200 <= r && r < 300 {
        ans = 300 - r;
    } else if 300 <= r && r < 400 {
        ans = 400 - r;
    }
    println!("{}", ans);
}
