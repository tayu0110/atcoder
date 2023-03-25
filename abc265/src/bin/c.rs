#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, g: [Chars; h]};

    let (mut nx, mut ny) = (0, 0);
    let mut ck = std::collections::HashSet::new();

    while !ck.contains(&(nx, ny)) {
        ck.insert((nx, ny));
        let mut ok = false;
        match g[ny][nx] {
            'U' => {
                if ny > 0 {
                    ny -= 1;
                } else {
                    ok = true;
                }
            }
            'D' => {
                if ny < h - 1 {
                    ny += 1;
                } else {
                    ok = true;
                }
            }
            'L' => {
                if nx > 0 {
                    nx -= 1;
                } else {
                    ok = true;
                }
            }
            'R' => {
                if nx < w - 1 {
                    nx += 1;
                } else {
                    ok = true;
                }
            }
            _ => {
                unreachable!();
            }
        }
        if ok {
            println!("{} {}", ny + 1, nx + 1);
            std::process::exit(0);
        }
    }

    println!("-1");
}
