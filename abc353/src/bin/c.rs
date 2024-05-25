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
        a: [usize; n],
    }
    let modulo = pow(10, 8);
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            ans = ans + (a[i] + a[j]) % modulo;
        }
    }
    println!("{}", ans);
}
