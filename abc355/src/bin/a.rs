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
        a: usize, b: usize,
    }
    //ansに1から3までの数字を入れる
    let mut ans = vec![1, 2, 3];
    if a == b {
        println!("-1");
    } else {
        // ans から a と b を取り除いたものが答え
        ans.retain(|&x| x != a && x != b);
        println!("{}", ans[0]);
    }
}
