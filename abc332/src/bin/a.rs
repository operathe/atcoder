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
        n: usize, s: usize, k: usize,
        pq: [(usize, usize); n]
    }
    let mut ans = 0;
    for (p, q) in pq {
        ans += p * q;
    }
    if ans >= s {
        println!("{}", ans);
    } else {
        println!("{}", ans + k);
    }
}
