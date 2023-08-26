use std::collections::{HashSet, VecDeque};

use proconio::*;

fn main() {
    input! {h: usize, w: usize, c: [marker::Chars; h]}

    let mut row = vec![vec![0; 26]; h];
    let mut col = vec![vec![0; 26]; w];

    for i in 0..h {
        for j in 0..w {
            let k = c[i][j] as usize - b'a' as usize;
            row[i][k] += 1;
            col[j][k] += 1;
        }
    }

    let mut nt_row = VecDeque::new();
    let mut nt_col = VecDeque::new();
    for i in 0..h {
        if row[i].iter().filter(|&&c| c > 0).count() == 1 {
            nt_row.push_back(i);
        }
    }
    for i in 0..w {
        if col[i].iter().filter(|&&c| c > 0).count() == 1 {
            nt_col.push_back(i);
        }
    }

    let mut remrow = (0..h).collect::<HashSet<_>>();
    let mut remcol = (0..w).collect::<HashSet<_>>();
    while !nt_row.is_empty() || !nt_col.is_empty() {
        if remcol.len() == 1 || remrow.len() == 1 {
            break;
        }
        let mut ncol = VecDeque::new();
        let mut nrow = VecDeque::new();
        while let Some(now) = nt_row.pop_front() {
            if !remrow.contains(&now) {
                continue;
            }
            remrow.remove(&now);
            if let Some(idx) = row[now].iter().position(|&v| v > 0) {
                for &i in remcol.iter() {
                    col[i][idx] -= 1;
                    if col[i].iter().filter(|&&v| v > 0).count() == 1 {
                        ncol.push_back(i);
                    }
                }
            }
        }
        while let Some(now) = nt_col.pop_front() {
            if !remcol.contains(&now) {
                continue;
            }
            remcol.remove(&now);
            if let Some(idx) = col[now].iter().position(|&v| v > 0) {
                for &i in remrow.iter() {
                    row[i][idx] -= 1;
                    if row[i].iter().filter(|&&v| v > 0).count() == 1 {
                        nrow.push_back(i);
                    }
                }
            }
        }

        nt_row = nrow;
        nt_col = ncol;
    }

    println!("{}", remcol.len() * remrow.len());
}
