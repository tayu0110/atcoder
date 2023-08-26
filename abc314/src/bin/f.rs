use itertools::Itertools;
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn dfs(now: usize, e: &[Modint], res: &mut [Modint], t: &Vec<Vec<usize>>) {
    res[now] += e[now];
    for &to in &t[now] {
        res[to] += res[now];
        dfs(to, e, res, t);
    }
    // eprintln!("res: {res:?}");
}

fn main() {
    input! {n: usize, p: [(usize, usize); n-1]}

    let mut size = vec![Modint::one(); n];
    let mut raw = vec![usize::MAX; n];
    let mut parlist = vec![usize::MAX; n];
    let mut e = vec![Modint::zero(); n];

    fn calc_par(p: usize, parlist: &mut Vec<usize>) -> usize {
        if parlist[p] == usize::MAX {
            return p;
        }

        let par = calc_par(parlist[p], parlist);
        parlist[p] = par;
        par
    }

    for (p, q) in p {
        let pp = calc_par(p - 1, &mut parlist);
        let pq = calc_par(q - 1, &mut parlist);
        // eprintln!("p: {p}, pp: {pp}, q: {q}, pq: {pq}");

        let inv = (size[pp] + size[pq]).inv();
        let ep = size[pp] * inv;
        let eq = size[pq] * inv;

        e[pp] = ep;
        e[pq] = eq;

        let len = parlist.len();
        parlist.push(usize::MAX);
        parlist[pp] = len;
        parlist[pq] = len;
        raw.push(usize::MAX);
        raw[pp] = len;
        raw[pq] = len;
        e.push(Modint::zero());
        size.push(size[pp] + size[pq]);
    }

    // eprintln!("raw: {raw:?}");

    let mut t = vec![vec![]; parlist.len()];
    for i in 0..parlist.len() {
        if raw[i] == usize::MAX {
            continue;
        }

        t[raw[i]].push(i);
    }

    // eprintln!("t: {t:?}");

    // eprintln!("e: {e:?}");

    let mut res = vec![Modint::zero(); parlist.len()];
    dfs(parlist.len() - 1, &e, &mut res, &t);

    println!("{}", res.iter().take(n).join(" "))
}
