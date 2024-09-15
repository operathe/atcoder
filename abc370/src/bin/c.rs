#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min, Ordering};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    let mut v = vec![];
    let n = s.len();
    for i in 0..n {
        if s[i] > t[i] {
            v.push(i);
        }
    }
    for i in (0..n).rev() {
        if s[i] < t[i] {
            v.push(i);
        }
    }
    println!("{}", v.len());
    for &i in &v {
        s[i] = t[i];
        println!("{}", s.iter().join(""));
    }
}
