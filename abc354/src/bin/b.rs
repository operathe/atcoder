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
        mut users: [(String, usize); n]


    }
    // usresのスコアの合計をsum_scoreに格納
    let mut sum_score = 0;
    for (_, score) in &users {
        sum_score += score;
    }
    // usersを辞書順にソート
    users.sort_by(|a, b| a.0.cmp(&b.0));
    // インデックスがsum_scoreをnで割った余りになるユーザーを出力
    println!("{}", users[sum_score % n].0);
}
