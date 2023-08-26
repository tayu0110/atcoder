use proconio::*;
use unionfind::UnionFind;

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m], c: [usize; n]}

    let mut uf = UnionFind::new(n);
    let mut t = vec![vec![]; n];
    for (a, b) in e {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);

        if c[a - 1] != c[b - 1] {
            uf.merge(a - 1, b - 1);
        }
    }

    for i in 0..n {
        for &to in &t[i] {
            if c[i] == c[to] && uf.is_same(i, to) {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
