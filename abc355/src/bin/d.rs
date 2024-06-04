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
        n: usize,
        mut lr: [(usize, usize); n]
    }
    lr.sort_by(|a, b| a.0.cmp(&b.0));
    let mut ans = 0;
    for i in 0..n {
        let mut max_r = lr[i].1;
        for j in i + 1..n {
            if lr[j].0 <= max_r {
                ans += 1;
                max_r = max_r.min(lr[j].1);
            } else {
                break;
            }
        }
    }
    println!("{}", ans);
}
// fn main() {
//     input! {
//         n: usize,
//         lr: [(usize, usize); n]
//     }
//     // lr の要素がそれぞれ、共通の区間を持つ組の個数を求める
//     let mut ans = 0;
//     for i in 0..n {
//         for j in i + 1..n {
//             if lr[i].0 <= lr[j].0 && lr[j].0 <= lr[i].1 {
//                 ans += 1;
//             } else if lr[j].0 <= lr[i].0 && lr[i].0 <= lr[j].1 {
//                 ans += 1;
//             }
//         }
//     }
//     println!("{}", ans);
// }
