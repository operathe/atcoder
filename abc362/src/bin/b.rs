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
        triangle: [(i32, i32); 3]
    }
    let mut ans = 0;
    for i in 0..3 {
        let mut a = triangle[i];
        let mut b = triangle[(i + 1) % 3];
        let mut c = triangle[(i + 2) % 3];
        a.0 -= c.0;
        a.1 -= c.1;
        b.0 -= c.0;
        b.1 -= c.1;
        if a.0 * b.0 + a.1 * b.1 == 0 {
            ans = 1;
        }
    }
    println!("{}", if ans == 1 { "Yes" } else { "No" });
}
