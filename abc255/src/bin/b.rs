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
        n: usize,
        k: usize,
        a: [usize; k],
        xy: [(i64, i64); n],
    }

    let mut res = 0i64;
    for &(x, y) in &xy {
        let mut cres = i64::MAX;
        for &ai in &a {
            let (ax, ay) = xy[ai - 1];
            cres = min(cres, (x - ax).pow(2) + (y - ay).pow(2));
        }
        res = max(res, cres);
    }

    println!("{:.12}", (res as f64).sqrt());
}
