#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    //sの要素を重複無しで取り出す
    // let mut set = HashSet::new();
    // for chars in s {
    //     set.insert(chars.iter().collect::<String>());
    // let mut set: HashSet<String> = s.iter().map(|chars| chars.iter().collect()).collect();

    //sに何回出現するかをカウントして、最大の名前を出力
    let mut map = HashMap::new();
    for chars in &s {
        let key = chars.iter().collect::<String>();
        *map.entry(key).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut max_name = "".to_string();
    for (name, count) in map {
        if count > max_count {
            max_count = count;
            max_name = name;
        }
    }
    println!("{}", max_name);
}
