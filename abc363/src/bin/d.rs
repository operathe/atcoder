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
        mut n: u64,
    }
    n -= 1;
    if n == 0 {
        println!("0");
        return;
    }
    let mut len = 0;
    loop {
        let count = match len {
            0 => 1,
            len => 10u64.pow((len - 1) / 2) * 9,
        };
        if n < count {
            break;
        }
        n -= count;
        len += 1;
    }
    let ans = format!("{}", n + 10u64.pow((len - 1) / 2));
    let ans = ans
        .chars()
        .chain(ans.chars().rev().skip(len as usize % 2))
        .collect::<String>();
    println!("{}", ans);
}
