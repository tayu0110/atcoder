use std::fmt::Write as _;

use proconio::*;

fn main() {
    input! {t: usize}

    let mut buf = String::new();
    for _ in 0..t {
        input! {n: usize, m: usize, x: [usize; n], y: [usize; n]}

        if !x.contains(&(n * m)) && !y.contains(&(n * m)) {
            writeln!(buf, "No").unwrap();
            continue;
        }

        let mut memo = vec![[usize::MAX; 2]; n * m + 1];
        for (i, &x) in x.iter().enumerate() {
            memo[x][0] = i;
        }
        for (i, &y) in y.iter().enumerate() {
            memo[y][1] = i;
        }

        let mut dr = vec![false; n];
        let mut dc = vec![false; m];
        let mut row = vec![];
        let mut col = vec![];
        let mut ok = vec![];
        let mut ret = vec![vec![0usize; m]; n];
        for i in (1..n * m + 1).rev() {
            let r = memo[i][0];
            let c = memo[i][1];
            if r < usize::MAX && c < usize::MAX {
                ret[r][c] = i;
                if !dr[r] {
                    dr[r] = true;
                    for &c in &col {
                        if ret[r][c] == 0 {
                            ok.push((r, c));
                        }
                    }
                    row.push(r);
                }
                if !dc[c] {
                    dc[c] = true;
                    for &r in &row {
                        if ret[r][c] == 0 {
                            ok.push((r, c));
                        }
                    }
                    col.push(c);
                }
            } else if r < usize::MAX {
            } else if c < usize::MAX {
            } else {
            }
        }
    }

    print!("{buf}")
}
