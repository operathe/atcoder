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
// 整数 N,M が与えられるので、 k=0∑N​ popcount(k&M) を 998244353 で割った余りを求めてください。
// ここで、popcount(x) は x の二進表記における 1 の個数を表します。
// & は 非負整数 a と非負整数 b とのビット単位 AND 演算の結果 x=a&b は次のように定義されます。x は全ての非負整数 k について以下の条件を満たすただ1 つの非負整数である。a を 2 進法で書き下した際の 2k の位と b を 2 進法で書き下した際の 2k の位が共に 1 なら、 x を 2 進法で書き下した際の 2k の位は1 である。そうでないとき、x を 2 進法で書き下した際の 2k の位は 0 である。例えば 3=11(2)​,5=101(2)​ なので、 3&5=1 となります。
fn count_bit_sums(n: usize, m: usize) -> usize {
    let mut ans = 0;
    let mod_num = 998244353;

    for bit in 0..60 {
        let mask = 1 << bit;
        if m & mask != 0 {
            let count = (n + 1) / (mask << 1) * mask
                + std::cmp::max(0, (n + 1) % (mask << 1)).saturating_sub(mask);
            ans = (ans + count) % mod_num;
        }
    }

    ans
}

fn main() {
    input! {
        n: usize, m: usize,
    }

    println!("{}", count_bit_sums(n, m));
}
