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
        s: Chars
    }
    //sの文字列のアルファベットを数えて、enumで管理する
    let mut count = [0; 26];
    for c in s {
        count[(c as u8 - b'a') as usize] += 1;
    }
    //最も多いアルファベットを出力する
    let max = count.iter().max().unwrap();
    let max_index = count.iter().position(|&x| x == *max).unwrap();
    println!("{}", (max_index as u8 + b'a') as char);
}
