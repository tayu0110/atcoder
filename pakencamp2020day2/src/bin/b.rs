#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {h: usize, w: usize, c: [Chars; h]};
    const INF: usize = 0x3f3f3f3f3f3f3f3f;

    let mut t = vec![vec![INF; w+1]; h+1];
    t[0][0] = 0;
    for i in 0..h {
        for j in 0..w {
            for (dx, dy, nc) in [(0, 1, 'E'), (1, 0, 'S')].iter() {
                if nc == &c[i][j] {
                    t[i+dx][j+dy] = std::cmp::min(t[i+dx][j+dy], t[i][j]);
                } else {
                    t[i+dx][j+dy] = std::cmp::min(t[i+dx][j+dy], t[i][j] + 1);
                }
            }
        }
    }

    println!("{}", t[h-1][w]);
}
