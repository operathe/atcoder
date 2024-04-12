use proconio::*;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n]
    }

    for row in a {
        for (x, cell) in row.iter().enumerate() {
            if *cell == 1 {
                print!("{} ", x + 1)
            }
        }
        println!()
    }
}
