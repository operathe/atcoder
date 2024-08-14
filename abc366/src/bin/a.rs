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
        t: usize,
        a: usize,
    }
    if t > (n / 2) {
        println!("Yes");
        return;
    } else if a > (n / 2) {
        println!("Yes");
        return;
    } else {
        println!("No");
        return;
    }
}
