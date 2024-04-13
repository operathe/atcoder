use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n:usize,
        s:Chars,
        q:usize,
        cd:[(char,char);q],
    }
    let mut map = HashMap::new();
    ('a'..='z').into_iter().for_each(|c| {
        map.insert(c, c);
    });
    for (c, d) in cd {
        let keys: Vec<_> = map
            .iter()
            .filter(|(_, &v)| v == c)
            .map(|(&k, _)| k)
            .collect();
        for key in keys {
            map.insert(key, d);
        }
    }
    //sを文字列で出力
    let ans = s.iter().map(|&c| map[&c]).collect::<String>();
    println!("{}", ans);
}
