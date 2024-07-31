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
        mut s: [String; n],
    }
    s.pop();
    let ans = if s.windows(2).any(|s| s[0] == s[1] && s[0] == "sweet") {
        "No"
    } else {
        "Yes"
    };
    println!("{}", ans);
}
