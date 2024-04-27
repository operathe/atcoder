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
        a: [usize; 9],
        b: [usize; 8],
    }
    //aの合計からbの合計を引いたものが答え
    let sum_a = a.iter().sum::<usize>();
    let sum_b = b.iter().sum::<usize>();
    println!("{}", sum_a - sum_b + 1);
}
