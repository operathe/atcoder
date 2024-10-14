#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use proconio::{fastout, input};
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: usize, // 機械の数
        x: usize, // 予算
        machines: [(usize, usize, usize, usize); n], // 各機械のパラメータ (a, p, b, q)
    }

    let mut max_capacity = 0;
    let mut left = 0;
    let mut right = 10_usize.pow(9); // 最大可能な生産能力

    // 二分探索で最大生産能力を求める
    while left <= right {
        let mid = (left + right) / 2;
        if can_achieve_capacity(n, x, &machines, mid) {
            max_capacity = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", max_capacity);
}

// 目標生産能力を達成できるかどうかを確認する関数
fn can_achieve_capacity(
    n: usize,
    x: usize,
    machines: &[(usize, usize, usize, usize)],
    target: usize,
) -> bool {
    let mut total_cost = 0;

    for &(a, p, b, q) in machines {
        // 生産能力 target を達成するためのコストを計算
        let cost_s = (target + a - 1) / a * p;
        let cost_t = (target + b - 1) / b * q;
        total_cost += min(cost_s, cost_t);

        // 予算を超えた場合は false を返す
        if total_cost > x {
            return false;
        }
    }

    true
}
