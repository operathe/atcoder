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
#[fastout]
fn main() {
    input! {
        n: usize, s: usize, m: usize, l: usize,
    }
    const INF: usize = std::usize::MAX;
    let mut ans = INF;
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if 6 * i + 8 * j + 12 * k >= n {
                    let temp = i * s + j * m + k * l;
                    if temp < ans {
                        ans = temp;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
