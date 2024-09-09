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
        mut a: [i64; n],
    }

    // 高橋君は n 匹のモンスターに順に出会います。
    // i 番目のモンスターの強さは a[i] です。
    // 高橋君は各モンスターを「拾う」か「捨てる」かを選べます。
    // 拾った場合、そのモンスターの強さ a[i] の経験値を得ます。
    // 拾う回数が偶数回目のとき、そのモンスターの強さ a[i] の2倍の経験値を得ます。
    // 捨てた場合は経験値を得られません。
    // 高橋君が得られる経験値の合計を最大化してください。
    // 制約: 1 <= n <= 2*10^5, 1 <= a[i] <= 10^9

    let mut dp = [0, i64::MIN];
    for a in &a {
        dp = [dp[0].max(dp[1] + a * 2), dp[1].max(dp[0] + a)];
    }
    println!("{}", dp[0].max(dp[1]));
}
