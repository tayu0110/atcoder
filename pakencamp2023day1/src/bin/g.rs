use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, mut a: [i64; n]}

    if a.iter().all(|&a| a < 0) {
        a.iter_mut().for_each(|a| *a = -*a);
    }

    if a.iter().all(|&a| a >= 0) {
        let &min = a.iter().min().unwrap();
        println!("{}", a.into_iter().fold(0, |s, v| s + v * min) - min * min);
        return;
    }

    a.sort_unstable();
    let mut e = vec![];
    for i in 0..n {
        let (w, b) = if a[i] < 0 {
            (a[i] * a.last().unwrap(), n - 1)
        } else {
            (a[i] * a[0], 0)
        };
        e.push((w, i, b));
    }
    e.sort_unstable();

    let mut uf = UnionFind::new(n);
    let mut res = 0;
    for (w, u, v) in e {
        if !uf.is_same(u, v) {
            res += w;
            uf.merge(u, v);
        }
    }

    println!("{}", res);
}
