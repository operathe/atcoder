#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[fastout]
fn calculate_a(a: &[usize]) -> usize {
    a.iter().fold(0, |acc, &x| acc | 1 << x)
}

fn calculate_ans(trials: &[(usize, bool)], n: usize, k: usize) -> usize {
    (0usize..1 << n)
        .filter(|bs| {
            trials
                .iter()
                .all(|&(a, r)| r == ((a & bs).count_ones() >= k.try_into().unwrap()))
        })
        .count()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let trials = (0..m)
        .map(|_| {
            input! {
                c: usize,
                a: [Usize1; c],
                r: char
            }
            let a = calculate_a(&a);
            (a, r == 'o')
        })
        .collect_vec();
    let ans = calculate_ans(&trials, n, k);
    println!("{}", ans);
}
