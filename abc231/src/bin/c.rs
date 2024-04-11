#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize, q: usize,
        mut a_vec: [usize; n],

    }
    //縦にq個の標準入力を受け取る
    let mut x_vec = Vec::new();
    for _ in 0..q {
        input! {
            x: usize,
        }
        x_vec.push(x);
    }
    a_vec.sort();
    //a_vecの中でx以上の値がいくつあるかを求める
    for x in x_vec {
        let mut left = 0;
        let mut right = n; // n とすることで、範囲を一つ広げる
        while left < right {
            let mid = left + (right - left) / 2;
            if a_vec[mid] < x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        let ans = if left < n && a_vec[left] >= x {
            n - left
        } else {
            0
        };
        println!("{}", ans);
    }
}
