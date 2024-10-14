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
        s: Chars,
        t: Chars
    }
    // sとtを比較して、一致しているかどうかを判定する
    // 一致している場合は0を出力する
    // 一致していない場合は、一番最初に異なる文字が出現する位置を出力する
    // ただし、sとtの長さが異なる場合は、短い方の文字列の長さを超える位置が異なる文字となる
    // そのため、短い方の文字列の長さを超える位置が異なる文字となる
    let len_s = s.len();
    let len_t = t.len();
    let min_len = min(len_s, len_t);

    for i in 0..min_len {
        if s[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }

    if len_s != len_t {
        println!("{}", min_len + 1);
    } else {
        println!("0");
    }
}
