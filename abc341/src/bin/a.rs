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
        n: isize,
    }
    let mut ans = vec![];
    for _ in 0..n {
        ans.push(1);
        ans.push(0);
    }
    ans.push(1);

    println!("{}", ans.iter().map(|&x| x.to_string()).collect::<String>());
}
