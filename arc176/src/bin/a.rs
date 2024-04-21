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
        n: usize, m: usize,
        ab: [(usize, usize); m]
    }

    let mut ans = vec![vec![0; n]; n];

    for (a, b) in ab {
        ans[a - 1][b - 1] = 1;
        ans[b - 1][a - 1] = 1; // 無向グラフなので双方向に設定
    }

    for i in 0..n {
        let mut sum = ans[i].iter().sum::<usize>(); // 行の合計を求める
        if sum < m {
            for j in 0..n {
                if ans[i][j] == 0 {
                    ans[i][j] = 1;
                    ans[j][i] = 1; // 無向グラフなので双方向に設定
                    sum += 1;
                    if sum == m {
                        break;
                    }
                }
            }
        }
    }

    for j in 0..n {
        let mut sum = ans.iter().map(|row| row[j]).sum::<usize>(); // 列の合計を求める
        if sum < m {
            for i in 0..n {
                if ans[i][j] == 0 {
                    ans[i][j] = 1;
                    ans[j][i] = 1; // 無向グラフなので双方向に設定
                    sum += 1;
                    if sum == m {
                        break;
                    }
                }
            }
        }
    }

    let mut count = 0;
    for i in 0..n {
        for j in 0..n {
            if ans[i][j] == 1 {
                count += 1;
            }
        }
    }
    println!("{}", count / 2); // 無向グラフなので辺を2回数えているので2で割る

    for i in 0..n {
        for j in i + 1..n {
            // 重複して出力しないように範囲を調整
            if ans[i][j] == 1 {
                println!("{} {}", i + 1, j + 1);
            }
        }
    }
}
