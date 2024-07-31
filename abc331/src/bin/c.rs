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
    let mut z = a.clone();
    z.sort();
    let mut sum = z.clone();
    sum.push(0);
    for i in (0..n).rev() {
        sum[i] += sum[i + 1];
    }
    println!(
        "{}",
        a.iter()
            .map(|a| {
                let x = z.upper_bound(a);
                sum[x]
            })
            .join(" ")
    );
}
