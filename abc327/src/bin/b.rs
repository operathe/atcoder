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
// fn main() {
//     input! {
//         b: u64,
//     }
//     let ans = (1..=15 as u64)
//         .find(|&x| x.pow(x as u32) == b)
//         .map_or(-1, |x| x as i64);
//     println!("{}", ans);
// }
fn main() {
    input! {
        b: i64,
    }
    for i in 1i64..=15 {
        if i.pow(i as u32) == b as i64 {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
