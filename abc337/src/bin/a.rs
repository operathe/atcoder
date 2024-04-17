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
        mut point: [(usize, usize); n]
    }
    let mut Takahashi = 0;
    let mut Aoki = 0;
    for i in 0..n {
        Takahashi += point[i].0;
        Aoki += point[i].1;
    }
    if Takahashi > Aoki {
        println!("Takahashi");
    } else if Takahashi < Aoki {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}
