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
//         n: usize,
//         s: String,
//     }
//     let ans = if s.find("ab").is_some() || s.find("ba").is_some() {
//         "Yes"
//     } else {
//         "No"
//     };
//     println!("{}", ans);
// }

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let ans = if s.contains("ab") || s.contains("ba") {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
