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
        n: usize, q: usize,
        t: [usize; q],
    }
    let mut ans = vec![1; n];
    for i in t {
        if ans[i - 1] == 1 {
            ans[i - 1] = 0;
        } else {
            ans[i - 1] = 1;
        }
    }
    // ansの中身を足し算して出力
    let mut sum = 0;
    for a in ans {
        sum += a;
    }
    println!("{}", sum);
}
