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
        lr: [(isize, isize); n],
    }

    let mut sum = 0;
    let mut result = Vec::with_capacity(n);

    // 最小値の合計を計算
    for &(l, _) in &lr {
        sum += l;
        result.push(l);
    }

    if sum > 0 {
        // 合計が正の場合、できるだけ小さくする
        for i in 0..n {
            let (l, r) = lr[i];
            if sum > 0 && result[i] > l {
                let diff = min(sum, result[i] - l);
                result[i] -= diff;
                sum -= diff;
            }
            if sum == 0 {
                break;
            }
        }
    } else if sum < 0 {
        // 合計が負の場合、できるだけ大きくする
        for i in 0..n {
            let (l, r) = lr[i];
            if sum < 0 && result[i] < r {
                let diff = min(-sum, r - result[i]);
                result[i] += diff;
                sum += diff;
            }
            if sum == 0 {
                break;
            }
        }
    }

    if sum == 0 {
        println!("Yes");
        println!("{}", result.iter().join(" "));
    } else {
        println!("No");
    }
}
