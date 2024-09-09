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
        positions: [(isize, String); n]
    }

    let ans = count_lr(positions);
    println!("{}", ans);
}

fn count_lr(positions: Vec<(isize, String)>) -> isize {
    let mut ans = 0;
    let mut last_r = None;
    let mut last_l = None;

    for &(pos, ref hand) in &positions {
        match hand.as_str() {
            "R" => {
                if let Some(last) = last_r {
                    ans += (last as isize - pos as isize).abs();
                }
                last_r = Some(pos);
            }
            "L" => {
                if let Some(last) = last_l {
                    ans += (last as isize - pos as isize).abs();
                }
                last_l = Some(pos);
            }
            _ => {}
        }
    }
    ans
}
