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
fn main() {
    input! {
        h: usize, w: usize,
        mut a: [[usize;w]; h]
    }
    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];
    // 行ごとの和を計算
    for i in 0..h {
        for j in 0..w {
            row_sum[i] += a[i][j];
        }
    }
    // 列ごとの和を計算
    for i in 0..w {
        for j in 0..h {
            col_sum[i] += a[j][i];
        }
    }
    // 答えを計算
    for i in 0..h {
        for j in 0..w {
            if j != 0 {
                print!(" ");
            }
            print!("{}", row_sum[i] + col_sum[j] - a[i][j]);
        }
        println!();
    }
}
