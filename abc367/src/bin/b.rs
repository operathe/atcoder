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
        x: String,
    }
    // 文字列の後ろから見ていき、0を見つけたらそれを削除する
    // 0以外の文字が見つかるまで削除を続ける
    // 0が全て削除できたら、残った文字列を出力する
    // .が残った場合はそれも削除する
    // ただし、文字列が0のみの場合は0を出力する
    let result = remove_zero(x);
    println!("{}", result);
}

fn remove_zero(mut x: String) -> String {
    while x.ends_with('0') {
        x.pop();
    }
    if x.ends_with('.') {
        x.pop();
    }
    if x.is_empty() {
        "0".to_string()
    } else {
        x
    }
}
