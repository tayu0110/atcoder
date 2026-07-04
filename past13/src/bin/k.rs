use proconio::*;
use rustc_hash::FxHashMap;

fn solve(y: usize, k: usize, memo: &mut FxHashMap<(usize, usize), usize>) -> usize {
    if y == 0 || k == 0 {
        return 0;
    }

    if let Some(&res) = memo.get(&(y, k)) {
        return res;
    }

    let mut res = 0;
    if k < 10 && k <= y {
        res = k;
    }

    for b in 0..10 {
        if y >= b && k >= b {
            res = res.max(solve((y - b) / 10, k - b, memo) * 10 + b);
        }
    }
    memo.insert((y, k), res);
    res
}

fn main() {
    input! {a: usize, b: usize, x: usize}

    let mut res = 0;
    for k in 1..=81 {
        if x >= b * k {
            res = res.max(solve(
                1000_000_000usize.min((x - b * k) / a),
                k,
                &mut FxHashMap::default(),
            ))
        }
    }
    println!("{}", res)
}
