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
        s: Chars,
        t: Chars,
    }
    let mut ans = vec![];
    let mut j = 0;
    for i in 0..s.len() {
        while j < t.len() {
            if s[i] == t[j] {
                ans.push(j + 1);
                j += 1;
                break;
            }
            j += 1;
        }
    }
    println!("{}", ans.iter().join(" "));
}
