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
        mut a :[[[u32; n]; n]; n],
        q: usize,
    }
    a.push(vec![vec![0; n]; n]);
    for a in &mut a {
        a.push(vec![0; n]);
    }
    for a in a.iter_mut().flatten() {
        a.push(0);
    }
    for (j, k) in iproduct!(0..=n, 0..=n) {
        for i in (0..n).rev() {
            a[i][j][k] += a[i + 1][j][k];
        }
    }
    for (i, k) in iproduct!(0..=n, 0..=n) {
        for j in (0..n).rev() {
            a[i][j][k] += a[i][j + 1][k];
        }
    }
    for (i, j) in iproduct!(0..=n, 0..=n) {
        for k in (0..n).rev() {
            a[i][j][k] += a[i][j][k + 1];
        }
    }
    for _ in 0..q {
        input! {
            i0: Usize1,
            i1: usize,
            j0: Usize1,
            j1: usize,
            k0: Usize1,
            k1: usize,
        }
        let ans = a[i0][j0][k0] - a[i0][j0][k1] - a[i0][j1][k0] - a[i1][j0][k0]
            + a[i0][j1][k1]
            + a[i1][j0][k1]
            + a[i1][j1][k0]
            - a[i1][j1][k1];
        println!("{}", ans);
    }
}
