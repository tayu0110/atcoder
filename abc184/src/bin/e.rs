#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, a: [Chars; h]}

    let mut t = vec![vec![]; 26];
    let mut start = (0, 0);
    let mut goal = (0, 0);
    for (i, v) in a.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if (&'a'..=&'z').contains(&c) {
                t[(*c as u8 - b'a') as usize].push((i, j));
            } else if *c == 'S' {
                start = (i, j);
            } else if *c == 'G' {
                goal = (i, j);
            }
        }
    }

    let mut res = vec![vec![std::usize::MAX; w]; h];

    let mut nt = std::collections::BinaryHeap::new();
    nt.push(std::cmp::Reverse((0, start)));
    while let Some(std::cmp::Reverse((nd, (y, x)))) = nt.pop() {
        if res[y][x] != std::usize::MAX {
            continue;
        }
        res[y][x] = nd;

        if y + 1 < h && a[y+1][x] != '#' && res[y+1][x] == std::usize::MAX {
            nt.push(std::cmp::Reverse((nd + 1, (y+1, x))));
        }
        if y > 0 && a[y-1][x] != '#' && res[y-1][x] == std::usize::MAX {
            nt.push(std::cmp::Reverse((nd + 1, (y-1, x))));
        }
        if x + 1 < w && a[y][x+1] != '#' && res[y][x+1] == std::usize::MAX {
            nt.push(std::cmp::Reverse((nd + 1, (y, x+1))));
        }
        if x > 0 && a[y][x-1] != '#' && res[y][x-1] == std::usize::MAX {
            nt.push(std::cmp::Reverse((nd + 1, (y, x-1))));
        }

        if 'a' <= a[y][x] && a[y][x] <= 'z' {
            let idx = (a[y][x] as u8 - b'a') as usize;

            for (ny, nx) in &t[idx] {
                if res[*ny][*nx] == std::usize::MAX {
                    nt.push(std::cmp::Reverse((nd + 1, (*ny, *nx))));
                }
            }
            t[idx].clear();
        }
    }

    if res[goal.0][goal.1] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", res[goal.0][goal.1]);
    }
}
