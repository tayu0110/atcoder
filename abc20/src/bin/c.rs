use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, t: usize, s: [Chars; h]};

    let ((sy, sx), (ty, tx)) = iproduct!(0..h, 0..w)
        .filter(|&(y, x)| s[y][x] == 'S' || s[y][x] == 'G')
        .collect_tuple()
        .unwrap();

    const INF: usize = 0x3f3f3f3f3f3f3f3f;
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];
    let mut l = 0usize;
    let mut r = INF;
    while r - l > 1 {
        let m = (r + l) / 2;

        let mut dist = vec![vec![INF; w]; h];
        let mut nt = std::collections::BinaryHeap::new();
        nt.push(std::cmp::Reverse((0, sx, sy)));
        while let Some(std::cmp::Reverse((nd, x, y))) = nt.pop() {
            if nd >= dist[y][x] {
                continue;
            }
            dist[y][x] = nd;
            for (dx, dy) in dx.iter().zip(dy.iter()) {
                let nx = x as i32 + *dx;
                let ny = y as i32 + *dy;
                if 0 <= nx && nx < w as i32 && 0 <= ny && ny < h as i32 {
                    let nnd = if s[ny as usize][nx as usize] == '#' {
                        nd + m
                    } else {
                        nd + 1
                    };
                    if nnd < dist[ny as usize][nx as usize] {
                        nt.push(std::cmp::Reverse((nnd, nx as usize, ny as usize)));
                    }
                }
            }
        }

        if dist[ty][tx] <= t {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", l);
}
