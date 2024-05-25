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
        h: usize
    }
    let mut a = 1;
    // 2のi乗がｈを超えるまで
    // iを出力
    let mut i = 1;
    while a <= h {
        a += pow(2, i);
        i += 1;
    }
    println!("{}", i);
}
