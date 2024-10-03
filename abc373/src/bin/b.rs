#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{pow, abs};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use superslice::*;
use std::collections::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input!{
        h: String,
        n: usize,
        w: Usize1,
        s: [Chars; n],
        mut plan: [(usize, usize); n]
    }
}
