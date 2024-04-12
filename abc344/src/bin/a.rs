use im_rc::hashmap::Values;
#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rand::seq::index;
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
        s: String,
    }
    //sを|で区切る
    let a: Vec<&str> = s.split("|").collect();
    let left = a[0].to_string();
    let right = a[2].to_string();
    println!("{}{}", left, right);
}
