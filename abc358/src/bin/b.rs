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
    n:usize, a:i64, t:[i64;n]
    }
    let mut now = 0;
    for i in 0..n {
        if now >= t[i] {
            now += a;
        } else {
            now = t[i] + a;
        }
        println!("{}", now);
    }
}
