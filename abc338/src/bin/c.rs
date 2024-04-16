use proconio::*;

fn main() {
    input! {
        n: usize,
        q: [u32; n],
        a: [u32; n],
        b: [u32; n],
    }
    let mut ans = 0;
    for i in 0.. {
        if q.iter().zip(a.iter()).any(|p| *p.0 < *p.1 * i) {
            break;
        }
        let k = q
            .iter()
            .zip(a.iter())
            .zip(b.iter())
            .filter(|p| *p.1 > 0)
            .map(|((q, a), b)| (*q - i * *a) / *b)
            .min()
            .unwrap();
        ans = ans.max(i + k);
    }
    println!("{}", ans);
}
