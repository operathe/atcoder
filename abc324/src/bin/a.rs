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
        a: [usize; n],
    }
    for i in 0..n - 1 {
        if a[i] != a[i + 1] {
            println!("No");
            return;
        }
    }
    print!("Yes");
}
