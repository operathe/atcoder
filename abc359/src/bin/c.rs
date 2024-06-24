#[allow(unused_imports)]
use ac_library::ModInt998244353;
#[allow(unused_imports)]
use itertools::*;
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::*};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use superslice::*;
#[allow(non_snake_case)]
#[allow(unused_variables)]

// 以下の問題
// 平面座標上に2×1のタイルが敷き詰められている
// 整数の組i(i,j)に対して、正方形A(x,y)= {(x,y)|i<=x<=i+1^j<=y<=j+1}は一つのタイルに含まれる
// i+jが偶数のとき、A(i,j)とA(i+1,j+1)は同じタイルに含まれる
// ただし、タイルは境界を含むこととし、共通部分が正の面積を持つような2つの異なるタイルは存在しない
// 私は、座標平面上のs(x+0.5,y+0.5)にいます
// 上下左右に移動することができ、異なるタイルを通るたびに1のコストがかかります
// t(x+0.5,y+0.5)に移動するための最小コストを求めてくださいj

fn main() {
    input! {
        mut s: (i64, i64),
        mut t: (i64, i64),
    }
    let mut ans = 0;
    if (s.0 + s.1) % 2 == 1 {
        s.0 -= 1;
    }
    if (t.0 + t.1) % 2 == 1 {
        t.0 -= 1;
    }
    if s.0 > t.0 {
        // s.0とt.0の位置を入れ替える
        std::mem::swap(&mut s.0, &mut t.0);
    }
    if s.1 > t.1 {
        // 互いの位置を入れ替える
        std::mem::swap(&mut s.1, &mut t.1);
    }
    // tの位置を(s.0-t.0, s.1-t.1)にする
    // sの位置を(0,0)にする
    t = (t.0 - s.0, t.1 - s.1);
    if t.0 <= t.1 {
        ans += t.1;
    } else {
        ans += t.1 + (t.0 - t.1) / 2;
    }

    println!("{}", ans);
}
