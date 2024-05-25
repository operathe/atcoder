#[allow(unused_imports)]
use proconio::{input, marker::*};
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut a: Vec<usize> = vec![0; n];
    let mut b: Vec<usize> = vec![0; n];
    for i in 0..n {
        let (ai, bi) = ab[i];
        a[i] = ai;
        b[i] = bi;
    }
    let sum_a: usize = a.iter().sum();
    let mut ans = 0;
    for (a, b) in ab {
        let sum_without_i = sum_a - a;
        ans = max(ans, sum_without_i + b);
    }
    println!("{}", ans);
}
