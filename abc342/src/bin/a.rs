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
        s:Chars
    }
    let ans = 0;
    //sの一文字目と二番目が異なり、かつ二番目と三番目が同じか判別する
    if s[0] != s[1] && s[1] == s[2] {
        println!("{}", ans + 1)
    } else {
        //一番目の文字と異なる文字を探す
        for i in 1..s.len() {
            if s[i] != s[0] {
                println!("{}", i + 1);
                return;
            }
        }
    }
}
