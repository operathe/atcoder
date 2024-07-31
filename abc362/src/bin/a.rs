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
        r: usize,
        g: usize,
        b: usize,
        c: String,
    }
    if c == "Red" {
        println!("{}", min(g, b));
    } else if c == "Green" {
        println!("{}", min(r, b));
    } else {
        println!("{}", min(r, g));
    }
}
