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
        n: usize
    }
    //nを２進数に変換
    let n_bin = format!("{:b}", n);
    //末尾に連続する0の数
    let mut count = 0;
    for c in n_bin.chars().rev() {
        if c == '0' {
            count += 1;
        } else {
            break;
        }
    }
    println!("{}", count);
}
