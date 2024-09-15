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
        x: [isize; n],
        p: [usize; n],
        q: usize,
        lr: [(isize, isize); q]
    }
    // 数直線上にn個の点があり、i番目の点の座標はx[i]である
    // また、i番目の点にはp[i]個のアイテムが置かれている
    // q個のクエリが与えられ、i番目のクエリはl[i]以上r[i]以下の区間にあるアイテムの個数を求める
    // クエリの答えを順番に出力する
    // 1 <= n,q <= 2*10^5
    // -10^9 <= x[i] <= 10^9
    // 1 <= p[i] <= 10^9
    // -10^9 <= l[i] <= r[i] <= 10^9

    // 座標とアイテム数をペアにして、座標でソートされたマップを作成
    let mut points = BTreeMap::new();
    for i in 0..n {
        *points.entry(x[i]).or_insert(0) += p[i];
    }

    // 累積和を計算
    let mut prefix_sum = vec![0];
    let mut sum = 0;
    for &count in points.values() {
        sum += count;
        prefix_sum.push(sum);
    }

    // 座標を配列に変換
    let coordinates: Vec<isize> = points.keys().cloned().collect();

    // クエリを処理
    for &(l, r) in &lr {
        let left_index = coordinates.partition_point(|&x| x < l);
        let right_index = coordinates.partition_point(|&x| x <= r);
        let item_count = prefix_sum[right_index] - prefix_sum[left_index];
        println!("{}", item_count);
    }
}
