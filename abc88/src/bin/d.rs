#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, s: [Chars; h]}

    let mut nt = std::collections::VecDeque::new();
    nt.push_back((0, 0, 0));

    let mut dist = vec![vec![std::usize::MAX; w]; h];
    while let Some((y, x, nd)) = nt.pop_front() {
        if dist[y][x] != std::usize::MAX {
            continue;
        }
        dist[y][x] = nd;

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            let ny = y as i32 + *dy;
            let nx = x as i32 + *dx;

            if nx < 0 || nx >= w as i32 || ny < 0 || ny >= h as i32 {
                continue;
            }

            if s[ny as usize][nx as usize] == '#' {
                continue;
            }

            nt.push_back((ny as usize, nx as usize, nd + 1));
        }
    }

    if dist[h-1][w-1] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", s.iter().flatten().filter(|c| **c == '.').count() - dist[h-1][w-1] - 1);
    }
}
