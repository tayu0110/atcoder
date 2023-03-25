#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn dfs(r: usize, c: usize, a: usize, i: usize, row_dir: &Vec<Vec<(usize, usize, usize)>>, col_dir: &Vec<Vec<(usize, usize, usize)>>, res: &mut [i64], row_memo: &mut std::collections::HashMap<(usize, usize), i64>, col_memo: &mut std::collections::HashMap<(usize, usize), i64>) -> i64 {
    if res[i] != std::i64::MAX {
        return res[i];
    }

    let mut max = -1;
    {
        let (mut l, mut u) = (0, row_dir[r].len());
        while u - l > 1 {
            let m = (u + l) / 2;
            let (b, _, _) = row_dir[r][m];

            if b > a {
                u = m;
            } else {
                l = m;
            }
        }

        if u != row_dir[r].len() {
            let (a, _, _) = row_dir[r][u];
            if let Some(&memo) = row_memo.get(&(r, a)) {
                max = std::cmp::max(max, memo);
            } else {
                let mut rmax = 0;
                for &(b, nc, j) in row_dir[r].iter().skip(u).take_while(|(b, _, _)| *b == a) {
                    let r = dfs(r, nc, b, j, row_dir, col_dir, res, row_memo, col_memo);
                    max = std::cmp::max(max, r);
                    rmax = std::cmp::max(rmax, r);
                }

                row_memo.insert((r, a), rmax);
            }

        }
    }

    {
        let (mut l, mut u) = (0, col_dir[c].len());
        while u - l > 1 {
            let m = (u + l) / 2;
            let (b, _, _) = col_dir[c][m];

            if b > a {
                u = m;
            } else {
                l = m;
            }
        }

        if u != col_dir[c].len() {
            let (a, _, _) = col_dir[c][u];
            if let Some(&memo) = col_memo.get(&(c, a)) {
                max = std::cmp::max(max, memo);
            } else {
                let mut rmax = 0;
                for &(b, nr, j) in col_dir[c].iter().skip(u).take_while(|(b, _, _)| *b == a) {
                    let r = dfs(nr, c, b, j, row_dir, col_dir, res, row_memo, col_memo);
                    max = std::cmp::max(max, r);
                    rmax = std::cmp::max(rmax, r);
                }

                col_memo.insert((c, a), rmax);
            }
        }
    }

    res[i] = max + 1;

    res[i]
}

fn main() {
    input! {h: usize, w: usize, n: usize, p: [(usize, usize, usize); n]}

    let mut row_dir = vec![vec![]; h+1];
    let mut col_dir = vec![vec![]; w+1];

    let mut v = vec![];
    for (i, (r, c, a)) in p.into_iter().enumerate() {
        row_dir[r].push((a, c, i));
        col_dir[c].push((a, r, i));

        v.push((a, r, c, i));
    }

    row_dir.iter_mut().filter(|v| !v.is_empty()).for_each(|v| v.sort());
    col_dir.iter_mut().filter(|v| !v.is_empty()).for_each(|v| v.sort());
    v.sort();

    let mut row_memo = std::collections::HashMap::new();
    let mut col_memo = std::collections::HashMap::new();

    let mut res = vec![std::i64::MAX; n];
    for (a, r, c, i) in v {
        if res[i] != std::i64::MAX {
            continue;
        }
    
        dfs(r, c, a, i, &row_dir, &col_dir, &mut res, &mut row_memo, &mut col_memo);
    }

    for res in res {
        println!("{}", res);
    }
}
