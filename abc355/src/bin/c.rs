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
        n: usize, t: usize,
        a: [usize; t],
    }

    let mut dp: Vec<Vec<_>> = (0..n * n)
        .map(|i| i + 1)
        .collect::<Vec<_>>()
        .chunks(n)
        .map(|chunk| chunk.to_vec())
        .collect();
    let mut rows = vec![0; n];
    let mut cols = vec![0; n];
    let mut diag = [0; 2];

    let mut positions = HashMap::new();
    for i in 0..n {
        for j in 0..n {
            positions.insert(dp[i][j], (i, j));
        }
    }

    for turn in 0..t {
        if let Some(&(i, j)) = positions.get(&a[turn]) {
            dp[i][j] = 0;
            rows[i] += 1;
            cols[j] += 1;
            if i == j {
                diag[0] += 1;
            }
            if i + j == n - 1 {
                diag[1] += 1;
            }

            if rows[i] == n || cols[j] == n || diag[0] == n || diag[1] == n {
                println!("{}", turn + 1);
                return;
            }
        }
    }

    println!("-1");
}
// fn main() {
//     input! {
//         n: usize, t: usize,
//         a: [usize; t],
//     }

//     let mut dp: Vec<Vec<_>> = (0..n * n)
//         .map(|i| i + 1)
//         .collect::<Vec<_>>()
//         .chunks(n)
//         .map(|chunk| chunk.to_vec())
//         .collect();
//     let mut rows = vec![0; n];
//     let mut cols = vec![0; n];
//     let mut diag = [0; 2];

//     let mut positions = HashMap::new();
//     for i in 0..n {
//         for j in 0..n {
//             positions.insert(dp[i][j], (i, j));
//         }
//     }

//     for turn in 0..t {
//         if let Some(&(i, j)) = positions.get(&a[turn]) {
//             dp[i][j] = 0;
//             rows[i] += 1;
//             cols[j] += 1;
//             if i == j {
//                 diag[0] += 1;
//             }
//             if i + j == n - 1 {
//                 diag[1] += 1;
//             }

//             if rows[i] == n || cols[j] == n || diag[0] == n || diag[1] == n {
//                 println!("{}", turn + 1);
//                 return;
//             }
//         }
//     }

//     println!("-1");
// }
