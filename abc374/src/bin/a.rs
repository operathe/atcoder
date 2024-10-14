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
        s: Chars
    }
    if s[s.len() - 1] == 'n' && s[s.len() - 2] == 'a' && s[s.len() - 3] == 's' {
        println!("Yes");
    } else {
        println!("No");
    }
}
