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
        k: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = a.len() - 1;

    while left < right {
        let mid = left + (right - left) / 2;
        match a[mid].cmp(&k) {
            std::cmp::Ordering::Equal => {
                println!("{}", mid);
                return;
            }
            std::cmp::Ordering::Greater => {
                right = mid;
            }
            std::cmp::Ordering::Less => {
                left = mid + 1;
            }
        }
    }

    let answer = if left < n - 1 { left as isize } else { -1 };
    println!("{}", answer);
}
