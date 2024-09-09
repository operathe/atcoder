#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min, Ordering};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }
    let mut v = vec![];
    let n = s.len();
    for i in 0..n {
        if s[i] > t[i] {
            v.push(i);
        }
    }
    for i in (0..n).rev() {
        if s[i] < t[i] {
            v.push(i);
        }
    }
    println!("{}", v.len());
    for &i in &v {
        s[i] = t[i];
        println!("{}", s.iter().join(""));
    }
}
// 英小文字からなる文字列sとtが与えられる
// sとtの長さは等しい
// 空の配列xを用意し、以下の操作とsとtが等しくなるまで繰り返す
// sの１文字を書き換えて、xの末尾に追加する
// こうして得られる文字列の配列 X の要素の個数と様子を出力してください。要素数最小のものが複数考えられる場合は、そのうち辞書順最小のものを求めてください。
// 辞書順で処理する
