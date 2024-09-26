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
        mut m: usize,
    }
    // i=1 から n までの3^aiの和がmになるnとaの組み合わせの数
    let mut a = vec![];

    for k in 0..11 {
        for _ in 0..(m % 3) {
            a.push(k);
        }
        m /= 3;
    }
    println!("{}", a.len());
    println!("{}", a.iter().join(" "));
}
