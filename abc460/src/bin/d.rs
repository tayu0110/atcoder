use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    if s.iter().flatten().all(|&b| b == b'#') {
        for _ in 0..h {
            println!("{}", ".".repeat(w));
        }
        return;
    }
    let mut nt = FxHashSet::default();
    let mut memo = vec![vec![-1; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'#' {
                memo[i][j] = 0;
                nt.insert((i, j));
            }
        }
    }

    let mut f = 0;
    while !nt.is_empty() {
        f = (f + 1) % 2;
        let mut next = FxHashSet::default();
        for &(r, c) in &nt {
            for (dr, dc) in [
                (0, 1),
                (1, 0),
                (0, !0),
                (!0, 0),
                (1, 1),
                (1, !0),
                (!0, 1),
                (!0, !0),
            ] {
                let nr = r.wrapping_add(dr);
                let nc = c.wrapping_add(dc);
                assert!(
                    nr >= h
                        || nc >= w
                        || nt.contains(&(nr, nc))
                        || memo[nr][nc] < 0
                        || memo[nr][nc] == f
                );

                if nr < h && nc < w && memo[nr][nc] < 0 {
                    // eprintln!("r: {r}, c: {c}, nr: {nr}, nc: {nc}, nf: {nf}");
                    next.insert((nr, nc));
                }
            }
        }
        for &(r, c) in &next {
            memo[r][c] = f;
        }
        nt = next;
    }

    for i in 0..h {
        let mut buf = String::with_capacity(w);
        for j in 0..w {
            if memo[i][j] == 0 {
                buf.push('#');
            } else {
                buf.push('.');
            }
        }
        println!("{buf}")
    }
}
