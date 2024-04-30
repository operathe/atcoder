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
    //空の配列に、右からAの要素を一つずつ追加していく
    //操作１ このとき配列にある要素が１つ以下ならば、操作を終了して次のAの要素を追加する
    //操作２ 配列にある要素のうち、右から１番目のものと右から２番目のものを比較して、大きさが異なるならば、操作を終了して次のAの要素を追加する
    //操作３ 配列にある要素のうち、右から１番目のものと右から２番目のものを比較して、大きさが同じならば、その２つを取り除き、取り除かれた様子の合計を追加し、操作１に戻って操作を行う。
    let mut b = Vec::new();
    for a in &a {
        b.push(*a);
        while b.len() > 1 && b[b.len() - 1] == b[b.len() - 2] {
            let last = b.pop().unwrap();
            b.pop().unwrap();
            b.push(last + 1);
        }
    }
    println!("{}", b.len());
}
