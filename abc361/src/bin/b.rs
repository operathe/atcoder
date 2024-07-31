#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::pow;
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        a: [isize; 6],
        b: [isize; 6],
    }
    let mut ans = false;
    if a[0].max(b[0]) < a[3].min(b[3])
        && a[1].max(b[1]) < a[4].min(b[4])
        && a[2].max(b[2]) < a[5].min(b[5])
    {
        ans = true;
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
