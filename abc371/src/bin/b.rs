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
        m: usize,
        first_child: [(usize, Chars); m]
    }
    // n個の家からm人の子供が生まれた
    // first_childは子供が生まれた家の番号と男女を表すM or F
    // first_childを最初から見ていき、それぞれの家で初めて生まれた男だけYesを出力。それ以外はNoを出力

    // 各家で子供が生まれたかどうかを追跡するベクター
    let mut has_child = vec![false; n + 1];

    // first_childの各エントリを順番に処理し、結果を出力
    for (house, gender) in first_child {
        if !has_child[house] && gender[0] == 'M' {
            println!("Yes");
            has_child[house] = true;
        } else {
            println!("No");
        }
    }
}
