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

fn reverse_range(n: usize, l: usize, r: usize) -> Vec<usize> {
    let mut ans: Vec<usize> = (1..=n).collect();
    ans[l - 1..r].reverse();
    ans
}

fn main() {
    input! {
        n: usize, l: usize, r: usize
    }
    let ans = reverse_range(n, l, r);
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
