use proconio::*;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.insert(0, 0);
    for i in 1..a.len() {
        a[i] += a[i - 1];
    }
    // println!("{:?}", a);
    let s = -*a.iter().min().unwrap();
    // println!("{:?}", s);
    let ans = a[n] + s;
    println!("{}", ans);
}
