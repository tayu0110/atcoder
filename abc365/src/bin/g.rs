use itertools::Itertools;
use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: usize, tp: [(u32, u32); m], q: usize, ab: [(u32, u32); q]}

    let mut memo = FxHashMap::default();
    let mut queries = vec![vec![]; n + 1];
    let mut parent = vec![usize::MAX; q];
    for (i, (a, b)) in ab.into_iter().enumerate() {
        if let Some(&index) = memo.get(&(a.min(b), a.max(b))) {
            parent[i] = index;
        } else {
            queries[a as usize].push((b, i as u32));
            queries[b as usize].push((a, i as u32));
            memo.insert((a.min(b), a.max(b)), i);
        }
    }

    let mut res = vec![0; q];
    let mut inout = vec![0; n + 1];
    for (t, p) in tp {
        if inout[p as usize] > 0 {
            for &(to, q) in queries[p as usize]
                .iter()
                .filter(|&q| inout[q.0 as usize] > 0)
            {
                res[q as usize] += t - inout[p as usize].max(inout[to as usize]);
            }
            inout[p as usize] = 0;
        } else {
            inout[p as usize] = t;
        }
    }

    for (i, p) in parent.into_iter().enumerate() {
        if p < usize::MAX {
            res[i] = res[p];
        }
    }

    println!("{}", res.iter().join("\n"));
}
