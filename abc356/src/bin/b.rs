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
// fn main() {
//     input! {
//         n: usize, m: usize,
//         s: [usize; m],
//         mut a: [[usize; m]; n]
//     }
//     let mut ans = vec![0; m];
//     for a in a {
//         for (i, &a) in a.iter().enumerate() {
//             ans[i] += a;
//         }
//     }
//     for (i, &s) in s.iter().enumerate() {
//         if ans[i] < s {
//             println!("No");
//             return;
//         }
//     }
//     println!("Yes");
// }
fn main() {
    input! {
        n: usize, m: usize,
        s: [usize; m],
        a: [[usize; m]; n]
    }
    let mut ans = vec![0; m];
    let mut is_possible = true;
    for (i, &s) in s.iter().enumerate() {
        for a in &a {
            ans[i] += a[i];
        }
        if ans[i] < s {
            is_possible = false;
            break;
        }
    }
    println!("{}", if is_possible { "Yes" } else { "No" });
}
