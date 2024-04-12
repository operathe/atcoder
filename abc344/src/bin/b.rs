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
    let mut ans = vec![];
    loop {
        input! {
            a: usize,
        }
        ans.push(a);
        if a.eq(&0) {
            break;
        }
    }
    for i in (0..ans.len()).rev() {
        println!("{}", ans[i]);
    }
}
