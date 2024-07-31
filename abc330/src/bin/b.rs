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
        l: usize,
        r: usize,
        a: [usize; n],
    }
    let mut ans = vec![];
    for &a in &a {
        if a < l {
            ans.push(l);
        } else if a > r {
            ans.push(r);
        } else {
            ans.push(a);
        }
    }
    println!("{}", ans.iter().join(" "));
}
