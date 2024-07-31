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
        a: [usize; n],
    }
    let mut ans = 0;
    for i in 1..=n {
        for j in 1..=a[i - 1] {
            // let s = format!("{}{}", i, j);
            // let mut set = HashSet::new();
            // for c in s.chars() {
            //     set.insert(c);
            // }
            // if set.len() == 1 {
            //     ans += 1;
            // }
            // iとjを文字列に変換して、HashSetに入れて、そのサイズが1だったらカウントする
            if format!("{}{}", i, j).chars().collect::<HashSet<_>>().len() == 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
