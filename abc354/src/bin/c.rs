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
        mut ac: [(usize, usize); n],
    }
    let mut ac = ac
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i, v.0, v.1))
        .collect::<Vec<_>>();
    ac.sort_by_key(|v| v.2);
    let mut ans = vec![];
    let mut now = 0;
    for (i, a, c) in ac {
        if now <= a {
            ans.push(i + 1);
            now = a;
        }
    }
    ans.sort();
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
    // let mut ans = ac
    //     .into_iter()
    //     .enumerate()
    //     .map(|(i, v)| (v.0, v.1, i))
    //     .collect::<Vec<_>>();
    // ans.sort_by_key(|v| v.1);
    // // println!("{:?}", ans);
    // let mut res = vec![];
    // let mut now = 0;
    // for (a, c, i) in ans {
    //     if now <= a {
    //         res.push(i + 1);
    //         now = a;
    //     }
    // }
    // res.sort();
    // println!("{}", res.len());
    // println!("{}", res.iter().join(" "));
}
