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
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    //aの要素を一文字ずつ並び替えて、[1,2,3,4,,,,,,n]にする。その際に並び替えた組と回数を出力する
    let mut swaps = Vec::new();
    let mut pos: Vec<usize> = vec![0; n];
    for i in 0..n {
        pos[a[i] - 1] = i;
    }
    for i in 0..n {
        if a[i] != i + 1 {
            let swap_idx = pos[i];
            a.swap(i, swap_idx);
            pos.swap(a[i] - 1, a[swap_idx] - 1);
            swaps.push((i + 1, swap_idx + 1));
        }
    }
    let count = swaps.len();
    println!("{}", count);
    for (i, j) in swaps {
        println!("{} {}", i, j);
    }
}
