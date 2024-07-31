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
    let mut i = 1;
    let mut ans = 0;
    while i * i * i <= n {
        let s = i * i * i;
        if s == reverse_number(s) {
            ans = s;
        }
        i += 1;
    }
    println!("{}", ans);
}

fn reverse_number(mut num: usize) -> usize {
    let mut rev = 0;
    while num > 0 {
        rev = rev * 10 + num % 10;
        num /= 10;
    }
    rev
}
// fn is_palindrome(n: u64) -> bool {
//     let s = n.to_string();
//     s.chars().eq(s.chars().rev())
// }
//
// fn largest_palindromic_cube(n: u64) -> u64 {
//     let cube_root = (n as f64).cbrt().ceil() as u64;
//
//     for i in (0..=cube_root).rev() {
//         let cube = i.saturating_mul(i).saturating_mul(i);
//         if cube <= n && is_palindrome(cube) {
//             return cube;
//         }
//     }
//
//     0
// }
