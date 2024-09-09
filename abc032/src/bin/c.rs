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
        s: [usize; n],
    }
    if s.iter().any(|&x| x == 0) {
        println!("{}", n);
        return;
    }
    let mut ans = 0;
    let mut right = 0;
    let mut tmp = 1;
    for left in 0..n {
        while right < n && tmp * s[right] <= k {
            tmp *= s[right];
            right += 1;
        }
        ans = max(ans, right - left);
        if left == right {
            right += 1;
        } else {
            tmp /= s[left];
        }
    }
    println!("{}", ans);
}
