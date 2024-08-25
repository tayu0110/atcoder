use proconio::*;
use unionfind::UnionFind;

const MAX: usize = 100010;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut uf = UnionFind::new(MAX * 2 + 10);
    for &(x, y) in &p {
        uf.merge(x, y + MAX);
    }

    let mut used = vec![false; MAX];
    let mut xs = vec![0usize; MAX * 2 + 10];
    for (x, _) in p {
        if !used[x] {
            xs[uf.root(x)] += 1;
            used[x] = true;
        }
    }

    let mut res = 0;
    for (i, xs) in xs.iter().enumerate().take(MAX * 2 + 10) {
        if *xs > 0 {
            let size = uf.size(i);
            let ys = size - xs;
            res += xs * ys;
        }
    }

    println!("{}", res - n);
}
