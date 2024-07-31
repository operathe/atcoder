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
    }
    // 非負整数xが回文数であるとは、xを十進法表記した際に左右対称であることを言います。
    // 例えば、12321は回文数であり、12345は回文数ではありません。
    // 正整数Nが与えられるので、0を１番目としたとき、N番目の回文数を求めてください。
    // ただし、回文数は0で始まることがあります。
    // 例えば、N=1のとき、0が答えとなります。
    // N番目の回文数を求める関数
    let result = nth_palindrome(n as u64);
    println!("{}", result);
}
fn nth_palindrome(n: u64) -> u64 {
    if n == 1 {
        return 0; // 1番目の回文数は0
    }

    let mut length = 1;
    let mut count = 9;
    let mut start = 1;
    let mut n = n;
    // N番目の回文数の桁数を決定
    while n > count {
        n -= count;
        length += 1;
        start *= 10;
        count = 9 * start;
        if length % 2 == 0 {
            count *= 10;
        }
    }

    // 左半分の数を生成
    let mut left = start + (n - 1) / (if length % 2 == 0 { 10 } else { 1 });

    // 回文数を構築
    let mut result = left;
    if length % 2 != 0 {
        left /= 10;
    }
    while left > 0 {
        result = result * 10 + left % 10;
        left /= 10;
    }

    result
}
