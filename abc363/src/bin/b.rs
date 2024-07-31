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
        t: isize,
        p: usize,
        mut l: [isize; n],
    }
    //lを降順にソート
    l.sort_by(|a, b| b.cmp(a));
    let ans = t - l[p - 1];
    println!("{}", if ans > 0 { ans } else { 0 });
}
