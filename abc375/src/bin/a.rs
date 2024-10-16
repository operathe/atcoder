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
        s: Chars,
    }

    let mut ans = 0;
    if n <= 2 {
        println!("{}", ans);
        return;
    }
    for i in 0..(n - 2) {
        if s[i] == '#' && s[i + 1] == '.' && s[i + 2] == '#' {
            ans += 1;
        }
    }
    println!("{}", ans);
}
