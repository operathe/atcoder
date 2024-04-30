#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    // 正五角形の頂点を表す文字列
    let pentagon = "ABCDE";
    // sとtが形成する線分の長さを計算
    let s_distance =
        (pentagon.find(s[0]).unwrap() as i32 - pentagon.find(s[1]).unwrap() as i32).abs();
    let t_distance =
        (pentagon.find(t[0]).unwrap() as i32 - pentagon.find(t[1]).unwrap() as i32).abs();
    // 線分の長さが等しいか判定
    if s_distance == t_distance {
        println!("Yes");
    } else {
        println!("No");
    }
}
