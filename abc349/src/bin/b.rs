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

    //sの中の文字をカウント
    let mut map = HashMap::new();
    for c in &s {
        *map.entry(c).or_insert(0) += 1;
    }
    //mapのvalueを全て取り出す
    let values: Vec<_> = map.values().collect();
    //valueの様子の出現回数をカウント
    let mut map2 = HashMap::new();
    for v in values {
        *map2.entry(v).or_insert(0) += 1;
    }
    //map2のvalueがすべて2ならYes
    if map2.values().all(|&k| k == 2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
