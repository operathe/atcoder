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
fn mod_pow(mut base: u128, mut exp: u128, mod_val: u128) -> u128 {
    let mut result = 1;
    base %= mod_val;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % mod_val;
        }
        base = base * base % mod_val;
        exp /= 2;
    }
    result
}

fn mod_inv(a: u128, module: u128) -> u128 {
    mod_pow(a, module - 2, module)
}

fn main() {
    input! {
        n: u128,
    }

    let mod_val = 998244353;
    let s = n.to_string();
    let k = s.len() as u128; // nの桁数
    let base = 10u128.pow(k as u32) % mod_val; // 10^k mod 998244353

    let r_pow_n = mod_pow(base, n, mod_val); // r^n mod 998244353
    let sum = (r_pow_n + mod_val - 1) % mod_val; // (r^n - 1) mod 998244353
    let inv_r_minus_1 = mod_inv(base - 1, mod_val); // (r - 1)^-1 mod 998244353
    let geometric_sum = sum * inv_r_minus_1 % mod_val; // 幾何級数の和
    let ans = geometric_sum * n % mod_val; // 最終的な答え

    println!("{}", ans);
}
