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
fn find_largest_power_of_two(mut n: usize) -> usize {
    let mut k = 0;
    while n > 1 {
        n /= 2;
        k += 1;
    }
    k
}

fn main() {
    // 入力を受け取る
    input! {
        mut l: usize, mut r: usize,
    }
    let mut intervals = Vec::new(); // 良い数列の区間を保存するためのベクター
    while l < r {
        let k = find_largest_power_of_two(l);
        let next_start = l + 2usize.pow(k as u32);
        intervals.push((l, next_start));
        l = next_start;
    }

    // 結果を出力する
    println!("Minimum number of intervals: {}", intervals.len());
    println!("Intervals:");
    for (l, r) in intervals {
        println!("({}, {})", l, r);
    }
}
