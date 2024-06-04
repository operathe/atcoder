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
        n: usize, m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }
    a.sort();
    // a と b の要素を一つの HashSet にまとめる
    let mut c: Vec<_> = a.iter().chain(&b).collect();
    c.sort();
    // aの配列の要素が2つ連続して、Cに含まれているかを判定する
    for i in 0..n + m - 1 {
        for j in 0..n - 1 {
            if c[i] == &a[j] && c[i + 1] == &a[j + 1] {
                println!("Yes");
                // println!("{:?}", c);
                return;
            }
        }
    }
    println!("No");
    // println!("{:?}", c);
}
