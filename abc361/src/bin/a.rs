#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
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
        n: usize, k: usize, x: usize,
        a: [usize; n],
    }
    //aのｋ番目の要素の次にxを挿入する
    let mut a = a;
    a.insert(k, x);
    println!("{}", a.iter().join(" "));
}
