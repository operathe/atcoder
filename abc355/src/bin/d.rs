#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::*};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(unused_imports)]
use superslice::*;
#[allow(non_snake_case)]
#[allow(unused_variables)]
#[fastout]
fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize) ; n],
    }
    let mut l = vec![];
    let mut r = vec![];
    for &(li, ri) in &lr {
        l.push(li);
        r.push(ri);
    }
    l.sort();
    r.sort();
    let mut res = 0;
    for &(li, ri) in &lr {
        res += r.lower_bound(&li);
        res += n - l.upper_bound(&ri);
    }
    println!("{}", n * (n - 1) / 2 - res / 2);
    println!("{:?}", l);
    println!("{:?}", r);
}
// fn main() {
//     input! {
//         n: usize,
//         mut lr: [(usize, usize); n]
//     }
//     lr.sort(); // 範囲の始点でソート
//     let mut ans = 0;
//     let mut end_points = VecDeque::new();
//     for &(l, r) in &lr {
//         while let Some(&first_end) = end_points.front() {
//             if first_end < l {
//                 end_points.pop_front();
//             } else {
//                 break;
//             }
//         }
//         end_points.push_back(r);
//         ans += end_points.len() - 1; // 自分自身を除く重なる範囲の数を加算
//     }
//     println!("{}", ans);
// }
