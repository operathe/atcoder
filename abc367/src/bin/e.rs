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
// fn main() {
//     input! {
//         n: usize,
//         k: usize,
//         x: [usize; n],
//         a: [usize; n],
//     }
//     // 各要素が1以上n以下である長さnの数列xと、長さnの数列aが与えられる
//     // aに以下の操作をk回行った後の数列xを求める
//     // 操作：b[i] = a[xi]となるようなb[i]を新たな数列aとする
//     // 1 <= n <= 2 * 10^5 0 <= k <= 10^18 1 <= x[i] <= n 1 <= a[i] <= 2 * 10^5
// }
fn transform(n: usize, mut k: usize, mut x: Vec<usize>, a: &[usize]) -> Vec<usize> {
    let mut seen = HashMap::new();
    let mut step = 0;

    while k > 0 {
        let state = x.clone();
        if let Some(&start) = seen.get(&state) {
            let cycle_length = step - start;
            let remaining = k % cycle_length;
            k = remaining;
            step = start;
            seen.clear();
        } else {
            seen.insert(state, step);
            let mut new_x = vec![0; n];
            for i in 0..n {
                new_x[i] = a[x[i] - 1];
            }
            x = new_x;
            step += 1;
            k -= 1;
        }
    }

    x
}

fn main() {
    input! {
        n: isize,
        k: isize,
        x: [isize; n],
        a: [isize; n],
    }

    let result = transform(n, k, x, a);
    for value in result {
        print!("{} ", value);
    }
    println!();
}
