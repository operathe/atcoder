use proconio::marker::*;
use proconio::*;
use std::collections::*;

type Deque<T> = VecDeque<T>;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut deq = (1..=n).map(|i| (i as i32, 0)).collect::<Deque<_>>();
    for _ in 0..q {
        input!(op: u32);
        if op == 1 {
            let mut f = deq[0];
            input!(c: String);
            match c.as_str() {
                "L" => f.0 -= 1,
                "R" => f.0 += 1,
                "U" => f.1 += 1,
                _ => f.1 -= 1,
            };
            deq.push_front(f);
            deq.pop_back();
        } else {
            input!(x: Usize1);
            let p = deq[x];
            println!("{} {}", p.0, p.1);
        }
    }
}
