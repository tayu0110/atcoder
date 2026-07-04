use proconio::*;
use rustc_hash::FxHashMap;

fn solve(ai: &mut [usize], aj: &mut [usize], ak: &mut [usize]) -> usize {
    let mut ret = 0usize;
    for &mut j in aj {
        let pi = ai.partition_point(|&i| i < j);
        let pk = ak.partition_point(|&k| k < j);

        ret += pi * pk;
        ret += (ai.len() - pi) * (ak.len() - pk);
    }
    ret
}

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut ai = FxHashMap::default();
    let mut aj = FxHashMap::default();
    let mut ak = FxHashMap::default();
    for (i, a) in a.into_iter().enumerate() {
        if a % 3 == 0 {
            ak.entry(a / 3).or_insert_with(|| vec![]).push(i);
        }
        if a % 5 == 0 {
            aj.entry(a / 5).or_insert_with(|| vec![]).push(i);
        }
        if a % 7 == 0 {
            ai.entry(a / 7).or_insert_with(|| vec![]).push(i);
        }
    }

    let mut keys = ak
        .keys()
        .chain(aj.keys())
        .chain(ai.keys())
        .copied()
        .collect::<Vec<_>>();
    keys.sort_unstable();
    keys.dedup();

    let mut ret = 0usize;
    for k in keys {
        if let (Some(ai), Some(aj), Some(ak)) = (ai.get_mut(&k), aj.get_mut(&k), ak.get_mut(&k)) {
            ret += solve(ai, aj, ak);
        }
    }

    println!("{ret}")
}
