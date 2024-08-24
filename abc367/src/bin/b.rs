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
        x: String,
    }
    let result = remove_zero(x);
    println!("{}", result);
}

fn remove_zero(mut x: String) -> String {
    while x.ends_with('0') {
        x.pop();
    }
    if x.ends_with('.') {
        x.pop();
        return x;
    } else {
        x
    }
}
