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
        q: usize,
        mut query: [(usize, usize ); q]
    }
    let mut ans = vec![];
    for (a, b) in query {
        //aが１ならansの最後にbを追加
        //aが２ならansの最後からb番目を出力
        if a == 1 {
            ans.push(b);
        } else {
            //ansの最後からb番目を出力
            println!("{}", ans[ans.len() - b]);
        }
    }
}
