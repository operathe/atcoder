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
        a: isize,
        b: isize,
    }
    let diff = a + b;
    match diff {
        diff if diff == 2 * a => println!("1"),
        diff if diff % 2 == 0 => println!("3"),
        _ => {
            println!("2");
        }
    }
}
