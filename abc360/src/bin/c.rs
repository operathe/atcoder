#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use std::mem::swap;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
        w: [usize; n],
    }
    let mut b = vec![vec![]; n];
    for i in 0..n {
        b[a[i]].push((w[i], i));
    }
    for i in 0..n {
        b[i].sort();
        b[i].reverse();
    }
    let mut ans = 0;
    for i in 0..n {
        if b[i].len() <= 1 {
            continue;
        }
        ans += b[i][1..].iter().map(|(x, _)| x).sum::<usize>();
    }
    println!("{}", ans);
}
