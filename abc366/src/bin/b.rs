#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use ac_library::*;
use itertools::*;
use num_traits::{abs, pow};
use proconio::{fastout, input, marker::*};
use std::cmp::{max, min};
use std::collections::*;
use superslice::*;

type Mint = ModInt998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }
    // s の長さの最大をmとする
    let m = s.iter().map(|x| x.len()).max().unwrap();

    //sを後ろの文字列から縦にm個並べる
    //このとき、sの長さがm未満の場合は、*を追加する
    let mut s = s
        .iter()
        .map(|x| {
            let mut x = x.clone();
            while x.len() < m {
                x.push('*');
            }
            x
        })
        .collect::<Vec<_>>();
    //sを縦に並べて、その文字列をansに格納する
    s.reverse();
    let mut ans = vec![];
    for i in 0..m {
        let mut tmp = vec![];
        for j in 0..n {
            tmp.push(s[j][i]);
        }
        ans.push(tmp);
    }
    // ansの文字列を後ろからみていって、アルファベットがでるまで*を削除する
    for row in &mut ans {
        while let Some(&'*') = row.last() {
            row.pop();
        }
    }

    println!(
        "{}",
        ans.iter()
            .map(|x| x.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}
