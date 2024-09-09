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

    // 長さnの整数数列aが与えられる
    // 1 <= l < r <= nを満たす全てのl, rについて、a[l]からa[r]までの部分列が等差数列になっているか判定
    // ありえる等差数列の数を出力 尺取り法

    let mut count = n;
    let mut left = 0;

    while left < n - 1 {
        let mut right = left + 1;
        let diff = a[right] - a[left];

        while right < n - 1 && a[right + 1] - a[right] == diff {
            right += 1;
        }

        let length = right - left + 1;
        count += length * (length - 1) / 2;

        left = right;
    }

    println!("{}", count);
}
