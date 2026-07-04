use std::collections::VecDeque;

use proconio::*;

const MAX: u32 = 5000000;
const D: [(usize, usize); 4] = [(0, 1), (1, 0), (0, !0), (!0, 0)];

fn dir2c(dir: usize) -> u8 {
    match dir {
        0 => b'R',
        1 => b'D',
        2 => b'L',
        3 => b'U',
        _ => unreachable!(),
    }
}

fn main() {
    input! {h: usize, w: usize, s: [marker::Bytes; h]}

    let mut start = (0, 0);
    let mut goal = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == b'S' {
                start = (i, j);
            } else if s[i][j] == b'G' {
                goal = (i, j);
            }
        }
    }

    let mut ret = vec![vec![[u32::MAX; 4]; w]; h];
    let mut prev = vec![vec![[(u32::MAX, u32::MAX, u32::MAX); 4]; w]; h];
    let mut nt = VecDeque::new();
    for i in 0..4 {
        nt.push_back((start.0, start.1, i));
        ret[start.0][start.1][i] = 0;
    }
    while let Some((r, c, dir)) = nt.pop_front() {
        if (r, c) == goal {
            break;
        }
        for (i, &(dr, dc)) in D.iter().enumerate() {
            if s[r][c] == b'o' && i != dir {
                continue;
            }
            if s[r][c] == b'x' && i == dir {
                continue;
            }
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr >= h || nc >= w {
                continue;
            }
            if s[nr][nc] == b'#' {
                continue;
            }
            if ret[nr][nc][i] < u32::MAX {
                continue;
            }

            ret[nr][nc][i] = ret[r][c][dir] + 1;
            prev[nr][nc][i] = (r as u32, c as u32, dir as u32);
            nt.push_back((nr, nc, i));
        }
    }

    let Some(mut dir) = ret[goal.0][goal.1].iter().position(|&r| r < MAX) else {
        println!("No");
        return;
    };
    eprintln!("ret: {}", ret[goal.0][goal.1][dir]);

    let mut stack = vec![];
    let (mut r, mut c) = goal;
    while ret[r][c][dir] != 0 {
        stack.push(dir2c(dir));
        let (pr, pc, pdir) = prev[r][c][dir];
        (r, c, dir) = (pr as usize, pc as usize, pdir as usize);
    }
    println!("Yes");
    println!(
        "{}",
        stack.iter().rev().map(|&c| c as char).collect::<String>()
    );
}
