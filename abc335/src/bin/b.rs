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
        n: usize
    }
    let mut v: Vec<(usize, usize, usize)> = vec![];
    // 合計がｎになるようなi,j,kの組み合わせを配列に格納
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k > n {
                    break;
                }
                v.push((i, j, k));
            }
        }
    }
    for (i, j, k) in v {
        println!("{} {} {}", i, j, k);
    }
}
