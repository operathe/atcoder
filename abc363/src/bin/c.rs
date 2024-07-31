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
        k: usize,
        mut s: Chars,
    }
    s.sort();
    let mut ans = 0usize;

    loop {
        let mut ok = true;
        for i in 0..=n - k {
            if (0..k).all(|j| s[i + j] == s[i + k - 1 - j]) {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
        if !s.next_permutation() {
            break;
        }
    }
    println!("{}", ans);
}
