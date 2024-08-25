use proconio::*;
use std::collections::VecDeque;

fn resolve(a: &mut [Vec<char>]) {
    const D: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];
    let h = a.len();
    let w = a[0].len();
    let mut nt = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                '>' => nt.push_back((i, j, 0)),
                '<' => nt.push_back((i, j, 2)),
                '^' => nt.push_back((i, j, 3)),
                'v' => nt.push_back((i, j, 1)),
                _ => {}
            }
        }
    }

    let mut reached = vec![vec![vec![false; 4]; w]; h];
    let bad = "><^v#";
    while let Some((r, c, di)) = nt.pop_front() {
        if reached[r][c][di] {
            continue;
        }
        reached[r][c][di] = true;

        let (dy, dx) = D[di];
        let (nr, nc) = (r.wrapping_add(dy), c.wrapping_add(dx));
        if nr < h && nc < w && !bad.contains(a[nr][nc]) && !reached[nr][nc][di] {
            nt.push_back((nr, nc, di));
        }
    }

    for i in 0..h {
        for j in 0..w {
            if reached[i][j].iter().any(|f| *f) {
                a[i][j] = '#';
            }
        }
    }
}

fn main() {
    input! {h: usize, w: usize, mut a: [marker::Chars; h]}

    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                s = (i, j)
            } else if a[i][j] == 'G' {
                g = (i, j);
            }
        }
    }
    resolve(&mut a);

    // for a in &a {
    //     eprintln!("{}", a.iter().collect::<String>())
    // }

    let mut nt = VecDeque::new();
    nt.push_back((s.0, s.1, 0));
    let mut reached = vec![vec![usize::MAX; w]; h];
    while let Some((r, c, nd)) = nt.pop_front() {
        if reached[r][c] < usize::MAX {
            continue;
        }
        reached[r][c] = nd;
        for (dx, dy) in [(0usize, 1usize), (1, 0), (0, !0), (!0, 0)] {
            let (nr, nc) = (r.wrapping_add(dy), c.wrapping_add(dx));

            if nr < h && nc < w && a[nr][nc] != '#' && reached[nr][nc] == usize::MAX {
                nt.push_back((nr, nc, nd + 1));
            }
        }
    }

    if reached[g.0][g.1] < usize::MAX {
        println!("{}", reached[g.0][g.1])
    } else {
        println!("-1")
    }
}
