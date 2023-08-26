use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut t = vec![vec![]; n];
    for (f, s) in p {
        t[f - 1].push(s);
    }
    let mut p = vec![];
    let mut res = 0;
    for v in t.iter_mut() {
        v.sort();

        let len = v.len();
        if !v.is_empty() {
            p.push(*v.last().unwrap());
        }

        if v.len() >= 2 {
            res = res.max(v[len - 1] + v[len - 2] / 2);
        }
    }

    p.sort();

    if p.len() >= 2 {
        res = res.max(p[p.len() - 1] + p[p.len() - 2]);
    }

    println!("{}", res)
}
