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
//         n: usize, k: usize,
//         a: [usize; n],
//     }
//     let mut ans = 0;
//     let mut pair = vec![];
//     for i in 0..n {
//         if ans + a[i] < k {
//             ans += a[i];
//             if i == n - 1 {
//                 pair.push(ans);
//             }
//         } else if ans + a[i] == k {
//             ans += a[i];
//             pair.push(ans);
//             ans = 0;
//         } else {
//             pair.push(ans);
//             ans = a[i];
//         }
//     }
//     println!("{:?}", pair);
//     println!("{}", pair.len());
// }

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }
    let mut ans = 0;
    let mut pair = vec![];
    for i in 0..n {
        if ans + a[i] <= k {
            ans += a[i];
        } else {
            pair.push(ans);
            ans = a[i];
        }
    }
    pair.push(ans);
    println!("{}", pair.len());
}
