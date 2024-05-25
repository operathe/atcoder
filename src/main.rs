use proconio::{input, marker::*};
use std::collections::VecDeque;
fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }
    // 探索済かの配列
    let mut visited = vec![vec![false; w]; h];
    // 深さ優先探索
    let mut que: VecDeque<(usize, usize)> = VecDeque::new();

    //　開始位置の探索
    for y in 0..h {
        for x in 0..w {
            if c[x][y] == 's' {
                visited[x][y] = true;
                que.push_back((x, y));
            }
        }
    }
    while let Some((x, y)) = que.pop_back() {
        if c[x][y] == 'g' {
            println!("Yes");
            return;
        }
        for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= w as i32 || ny < 0 || ny >= h as i32 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;
            if visited[nx][ny] {
                continue;
            }
            visited[nx][ny] = true;
            que.push_back((nx, ny));
        }
    }
    println!("No");
}
