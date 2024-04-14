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
        mut a: [u64; n],
        st: [(u64, u64); n - 1],
    };
    for (i, (s, t)) in st.into_iter().enumerate() {
        a[i + 1] += (a[i] / s) * t; // Remove the unnecessary dereference operator `*&` when accessing `i`
    }
    //aのさいごの要素を出力
    println!("{}", a[n - 1]);
}
