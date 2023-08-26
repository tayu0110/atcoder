use graph::scc;
use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut t = vec![vec![]; n];
    for i in 0..n {
        t[i].push(a[i] - 1);
    }

    let g = t.into();
    let c = scc(&g);

    let mut res = n;
    for v in c {
        if v.len() == 1 && a[v[0]] != v[0] + 1 {
            res -= 1;
        }
    }

    println!("{}", res)
}
