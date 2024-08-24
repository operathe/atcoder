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
        k: usize,
        x: [Usize1; n],
        a: [Usize1; n],
    }
    // 各要素が1以上n以下である長さnの数列xと、長さnの数列aが与えられる
    // aに以下の操作をk回行った後の数列xを求める
    // 操作：b[i] = a[xi]となるようなb[i]を新たな数列aとする
    // 1 <= n <= 2 * 10^5 0 <= k <= 10^18 1 <= x[i] <= n 1 <= a[i] <= 2 * 10^5
    let x = slove(&x, k);
    let mut ans = vec![0usize; n];
    for i in 0..n {
        ans[i] = a[x[i]] + 1;
    }
    println!("{}", ans.iter().join(" "));
}

fn slove(x: &Vec<usize>, k: usize) -> Vec<usize> {
    let n = x.len();
    let mut y = (0..n).collect_vec();
    let mut x = x.clone();
    let mut k = k;

    let step = |x: &Vec<usize>, y: &Vec<usize>| {
        let mut ret = vec![];
        for i in 0..n {
            ret.push(y[x[i]]);
        }
        ret
    };

    while k > 0 {
        if k % 2 == 1 {
            y = step(&x, &y);
        }
        x = step(&x, &x);
        k >>= 1;
    }
    y
}
