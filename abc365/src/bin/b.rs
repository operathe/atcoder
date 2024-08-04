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
    // aの要素で二番目に大きい要素を求める
    // このとき、元の配列のインデックスを出力する
    let mut a = a.into_iter().enumerate().collect::<Vec<_>>();
    a.sort_by_key(|x| x.1);
    println!("{}", a[n - 2].0 + 1);
}
