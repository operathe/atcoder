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
    }
    let ans = (n..=919)
        .find(|a| {
            let hundred = a / 100;
            let ten = a / 10 % 10;
            let one = a % 10;
            handred * ten == one
        })
        .unwrap();
    println!("{}", ans);
    // for a in n.. {
    //     let k100 = a / 100;
    //     let k10 = a / 10 % 10;
    //     let k = a % 10;
    //     if k100 * k10 == k {
    //         println!("{}", a);
    //         return;
    //     }
    // }
}
