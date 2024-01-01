use itertools::Itertools;
use proconio::*;

fn change_count(mut v: Vec<usize>) -> usize {
    let n = v.len();
    let mut res = 0;
    for i in 0..n {
        if v[i] != i {
            let pos = v.iter().position(|p| p == &i).unwrap();
            for j in (i..pos).rev() {
                v.swap(j, j + 1);
                res += 1;
            }
        }
    }

    res
}

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h], b: [[usize; w]; h]}

    let mut res = usize::MAX;
    for v in (0..h).permutations(h) {
        for w in (0..w).permutations(w) {
            let mut buf = vec![];
            for &v in &v {
                buf.push(a[v].clone());
            }

            let mut na = buf.clone();
            for i in 0..h {
                for (j, &w) in w.iter().enumerate() {
                    na[i][j] = buf[i][w];
                }
            }

            if na == b {
                res = res.min(change_count(v.clone()) + change_count(w));
            }
        }
    }

    println!("{}", res as i64)
}
