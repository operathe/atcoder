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
    }
    let mut ans = 1;
    for i in 1..=n {
        let cube = i.pow(3);
        if cube > n {
            break;
        }
        if is_palindrome(cube) {
            ans = cube;
        }
    }
    println!("{}", ans);
}

fn is_palindrome(n: usize) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}
