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
        m: usize,
        k: usize,
    }
    let mut tests = vec![];
    for _ in 0..m {
        input! {
            c: usize,
            a: [Usize1; c],
            r: char
        }
        tests.push((a, r));
    }
    let mut ans = 0;
    for s in 0..1 << n {
        let mut ok = true;
        for (a, r) in &tests {
            let mut count = 0;
            for &i in a {
                if s & (1 << i) != 0 {
                    count += 1;
                }
            }
            if (*r == 'o' && count < k) || (*r == 'x' && count >= k) {
                ok = false;
                break;
            }
        }
        if ok {
            ans += 1;
        }
    }
    println!("{}", ans);
}

// fn main() {
//     input! {
//         n: usize,
//         m: usize,
//         k: usize,
//     }
//     let trials = (0..m)
//         .map(|_| {
//             input! {
//                 c: usize,
//                 a: [Usize1; c],
//                 r: char
//             }
//             let a: u32 = a.iter().fold(0, |acc, &x| acc | 1 << x);
//             (a, r == 'o')
//         })
//         .collect_vec();
//     let ans = (0..1 << n)
//         .filter(|bs| {
//             trials
//                 .iter()
//                 .all(|&(a, r)| r == ((a & bs).count_ones() >= k as u32))
//         })
//         .count();
//     println!("{}", ans);
// }
