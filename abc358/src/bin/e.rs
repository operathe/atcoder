#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
use rand_core::le;
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
        k: usize,
        c: [usize; 26],
    }
    //a-zの26文字からできたタイルがあります。
    //タイルを一列に並べて、文字列を作ります。
    //k=<1000
    //c=<1000
    //i番目のタイルは、文字列のi番目のアルファベットを表します。
    //文字列の中に含まれているi番目のアルファベットの個数はciです。
    //長さ１以上ｋ以下の文字列であって上の条件を満たすものの個数を求めて、998244353で割った余りを出力してください。
    // 与えられたアルファベットの個数ciを格納する配列（例として初期化）

    let mod_val = 998244353;
    let mut ans = 1;

    for &ci in &c {
        let mut count = 0;
        for j in 0..=ci {
            count = (count + comb(k + j - 1, j, mod_val)) % mod_val;
        }
        ans = ans * count % mod_val;
    }

    println!("{}", ans);
}

fn comb(n: usize, r: usize, mod_val: i64) -> i64 {
    if r == 0 || r == n {
        return 1;
    }
    let mut numerator = 1;
    let mut denominator = 1;
    for i in 0..r {
        numerator = numerator * (n - i) as i64 % mod_val;
        denominator = denominator * (i + 1) as i64 % mod_val;
    }
    numerator * mod_inverse(denominator, mod_val) % mod_val
}

fn mod_inverse(a: i64, mod_val: i64) -> i64 {
    let mut m0 = mod_val;
    let mut y = 0;
    let mut x = 1;

    if mod_val == 1 {
        return 0;
    }

    let mut a = a;
    while a > 1 {
        let q = a / m0;
        let t = m0;
        m0 = a % m0;
        a = t;
        let t = y;
        y = x - q * y;
        x = t;
    }

    if x < 0 {
        x += mod_val;
    }

    x
}
