#![allow(unused_imports, non_snake_case, unused_variables, dead_code)]
use ac_library::ModInt998244353;
use itertools::*;
use num_traits::pow;
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;
type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = true;
    if s[0] == 'R' || (s[1] == 'R' && s[2] == 'R') {
        println!("Yes");
    } else {
        println!("No");
    }
}
