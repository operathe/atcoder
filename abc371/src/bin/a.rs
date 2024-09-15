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
        s: [String; 3],
    }
    // s0はAとBの大小関係
    // s1はAとCの大小関係
    // s2はBとCの大小関係
    // 二番目に大きいものを探す
    let mut order = HashMap::new();
    order.insert('A', 0);
    order.insert('B', 0);
    order.insert('C', 0);

    // AとBの関係
    if s[0] == ">" {
        *order.get_mut(&'A').unwrap() += 1;
    } else {
        *order.get_mut(&'B').unwrap() += 1;
    }

    // AとCの関係
    if s[1] == ">" {
        *order.get_mut(&'A').unwrap() += 1;
    } else {
        *order.get_mut(&'C').unwrap() += 1;
    }

    // BとCの関係
    if s[2] == ">" {
        *order.get_mut(&'B').unwrap() += 1;
    } else {
        *order.get_mut(&'C').unwrap() += 1;
    }

    // 順序に基づいてソート
    let mut sorted_order: Vec<_> = order.into_iter().collect();
    sorted_order.sort_by(|a, b| b.1.cmp(&a.1));

    // 二番目に大きい文字を出力
    println!("{}", sorted_order[1].0);
}
