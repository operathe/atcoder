fn main() {
    proconio::input!(mut n: u128);
    n -= 1;
    let mut ans = 0u128;
    let mut pow = 1;
    while n > 0 {
        ans += pow * (n % 5 * 2);
        n /= 5;
        pow *= 10;
    }
    println!("{}", ans);
}
// use itertools::Itertools;
// use proconio::input;
// fn main() {
//     input! {
//         mut n: usize
//     }
//     if n == 1 {
//         println!("0");
//     }
//     let mut ans = vec![];
//     n -= 1;
//     while n > 0 {
//         let index = n % 5;
//         ans.push(index * 2);
//         n /= 5;
//     }
//     println!("{}", ans.iter().rev().join(""));
//}
