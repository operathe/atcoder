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
        s: Chars,
    }
    //sの文字列の先頭が大文字かどうか判定する
    if s[0].is_uppercase() {
        if s[1..].iter().all(|&c| c.is_lowercase()) {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
