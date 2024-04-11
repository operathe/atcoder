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

#[fastout]

fn main() {
    input! {
        n: usize,
        mut tasks: [(usize, usize); n],
    }

    // タスクを終了日に基づいてソート
    tasks.sort_by_key(|&(_, b)| b);

    let mut count = 0;
    let mut current_end = 0;

    for (a, b) in tasks {
        if a >= current_end {
            count += 1;
            current_end = b;
        }
    }

    println!("{}", count);
}
