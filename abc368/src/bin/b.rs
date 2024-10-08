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
        mut a: [usize; n],
    }
    let mut heap = BinaryHeap::from(a);
    let mut ans = 0;

    while let Some(mut max1) = heap.pop() {
        if let Some(mut max2) = heap.pop() {
            if max2 == 0 {
                break;
            }
            ans += 1;
            max1 -= 1;
            max2 -= 1;
            heap.push(max1);
            heap.push(max2);
        } else {
            break;
        }
    }

    println!("{}", ans);
}
