use proconio::{input, marker::*};
fn main() {
    input! {
        n: usize,
        v: [i32; n],
        c: [i32; n],
    }

    let mut ans = 0;

    // 2^n 通りのbit全探索
    for bit in 0..1 << n {
        //ある組み合わせにおける x - y の値を保持する
        let mut total = 0;

        // j番目のbitが立っているかどうかを判定する
        for i in 0..n {
            // bitが立っている場合は x - y の値を加算する
            if bit & (1 << i) > 0 {
                total += v[i] - c[i];
            }
        }

        if ans < total {
            ans = total;
        }
    }
    println!("{}", ans);
}
// fn main() {
//     input! {
//         n: usize,
//         v: [usize; n],
//         c: [usize; n],
//     }
//     let mut ans = 0;
//     for i in 0..n {
//         if v[i] > c[i] {
//             ans += v[i] - c[i];
//         }
//     }
//     println!("{}", ans);
// }
