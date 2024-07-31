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
        mut points: [(isize, isize); n]
    }
    //各点の距離を求め、それを元に各点からの最短の点のインデックスを求める
    for (i, x) in points.iter().enumerate() {
        let mut max = 0;
        let mut index = 0;
        for (j, y) in points.iter().enumerate() {
            if i != j {
                let mut dist = (x.0 - y.0).pow(2) + (x.1 - y.1).pow(2);
                if dist > max {
                    max = dist;
                    index = j + 1;
                }
            }
        }
        println!("{}", index);
    }
}
