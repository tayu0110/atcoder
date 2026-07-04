use std::mem::take;

use proconio::*;
use rustc_hash::FxHashSet;

fn main() {
    input! {n: usize, m: usize, mut e: [(usize, usize); m], q: usize, x: [usize; q]}

    let mut t = vec![FxHashSet::default(); n];
    for (u, v) in e.iter_mut() {
        *u -= 1;
        *v -= 1;
        t[*u].insert(*v);
        t[*v].insert(*u);
    }
    let mut res = m;
    let mut par = vec![-1i32; n];
    fn root(u: usize, par: &mut [i32]) -> usize {
        if par[u] < 0 {
            return u;
        }
        par[u] = root(par[u] as usize, par) as i32;
        par[u] as usize
    }
    for x in x {
        let (u, v) = e[x - 1];
        let mut pu = root(u, &mut par);
        let mut pv = root(v, &mut par);

        if pu != pv && t[pu].contains(&pv) {
            let mut eu = take(&mut t[pu]);
            let mut ev = take(&mut t[pv]);
            if eu.len() < ev.len() {
                (eu, ev) = (ev, eu);
                (pu, pv) = (pv, pu);
            }
            par[pv] = pu as i32;
            for to in ev {
                if to == pu || to == pv {
                    res -= 1;
                    eu.remove(&to);
                    continue;
                }
                if !eu.insert(to) {
                    res -= 1;
                }
                t[to].remove(&pv);
                t[to].insert(pu);
            }
            t[pu] = eu;
        }

        println!("{res}")
    }
}
