use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n + 1];
    for (i, (p, y)) in p.into_iter().enumerate() {
        t[p].push((y, i));
    }

    let mut res = vec!["".to_string(); m];
    for (p, mut v) in t.into_iter().enumerate() {
        if v.is_empty() {
            continue;
        }

        v.sort();

        for (j, (_, i)) in v.into_iter().enumerate() {
            res[i] = format!("{:06}{:06}", p, j + 1);
        }
    }

    for r in res {
        println!("{}", r);
    }
}
