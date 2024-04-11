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
            r:usize,
            c:usize,

    }
    //標準入力から1 0　0 1と受け取って、二次元配列を作る
    let mut a = Vec::new();
    for _ in 0..r {
        input! {
            x:[usize;2],
        }
        a.push(x);
    }
    println!("{}", a[r - 1][c - 1])
}
