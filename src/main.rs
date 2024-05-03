use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n]
    }

    let mut count = 0;
    while a.iter().all(|&x| x % 2 == 0) {
        a = a.iter().map(|&x| x / 2).collect();
        count += 1;
    }
    println!("{}", count);
}
