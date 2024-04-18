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
        n: usize,
        a: [i32; n],
    }
    let mut next = vec![n; n];
    for (i, a) in a.iter().enumerate() {
        if *a != -1 {
            next[*a as usize - 1] = i;
        }
    }
    let mut ans = vec![];
    let mut pos = a.iter().position(|a| *a == -1).unwrap();
    while pos != n {
        ans.push(pos + 1);
        pos = next[pos];
    }
    println!("{}", ans.iter().join(" "));
}
