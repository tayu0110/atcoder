use proconio::*;
use rustc_hash::FxHashSet;

const MAX: usize = 1000_001;

fn main() {
    input! {n: usize, d: usize, mut e: [(usize, usize); n]}
    e.retain(|e| e.1 - e.0 >= d);

    let mut ins = vec![vec![]; MAX];
    let mut outs = vec![vec![]; MAX];
    for (i, (s, t)) in e.into_iter().enumerate() {
        ins[s].push(i);
        outs[t].push(i);
    }

    let mut ret = 0usize;
    let mut set = FxHashSet::default();
    let mut ok = FxHashSet::default();
    for i in 1..MAX {
        for &j in &ins[i] {
            set.insert(j);
        }

        if d <= i {
            for &j in &ins[i - d] {
                if set.contains(&j) {
                    ok.insert(j);
                }
            }
        }

        let len = ok.len();
        if len > 0 {
            ret += len * (len - 1) / 2;
        }

        for &j in &outs[i] {
            set.remove(&j);
            ok.remove(&j);
        }
    }
    println!("{}", ret);
}
