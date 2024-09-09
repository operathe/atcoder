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
        l: usize,
        r: usize,
    }
    if l == 1 && r != 1 {
        println!("Yes");
    } else if l != 1 && r == 1 {
        println!("No");
    } else {
        println!("Invalid");
    }
}
