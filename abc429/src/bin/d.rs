use proconio::*;
use rustc_hash::FxHashMap;

fn main() {
    input! {n: usize, m: usize, c: usize, a: [usize; n]}

    let mut map = FxHashMap::default();
    for a in a {
        *map.entry(a).or_insert(0usize) += 1;
    }

    let mut pos = map.into_iter().collect::<Vec<_>>();
    pos.sort_unstable();
    if pos[0].0 != 0 {
        pos.insert(0, (0, 0));
    }

    let n = pos.len();
    let mut cum = vec![0; 2 * n];
    for (i, &(_, c)) in pos.iter().enumerate() {
        cum[i] = c;
        cum[i + n] = c;
    }
    for i in 0..2 * n - 1 {
        cum[i + 1] += cum[i];
    }

    pos.push((m, 0));
    let mut ret = 0usize;
    // eprintln!("pos: {pos:?}, cum: {cum:?}");
    for (i, v) in pos.windows(2).enumerate() {
        let prev = v[0].0;
        let next = v[1].0;

        let sum = cum[n - 1];
        let rc = c % sum;
        let pos = cum.partition_point(|&cu| cu.saturating_sub(cum[i]) < rc);
        // eprintln!("i: {i}, sum: {sum}, c: {c}, pos: {pos}");
        ret += (c / sum * sum + cum[pos].saturating_sub(cum[i])) * (next - prev);
    }

    println!("{}", ret);
}
