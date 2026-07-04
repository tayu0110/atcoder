use proconio::*;
use rustc_hash::FxHashMap;

fn rec(
    n: usize,
    l: usize,
    r: usize,
    fl: bool,
    query: &[(char, usize)],
    memo: &mut FxHashMap<(usize, usize, usize), usize>,
) -> usize {
    if query.is_empty() {
        return 0;
    }

    let len = query.len();
    if let Some(&res) = memo.get(&(len, l, r)) {
        return res;
    }

    let (h, mut t) = query[0];
    t -= 1;

    let (nl, mut nr, nfl) = if (h == 'L') == fl {
        (l, r, fl)
    } else {
        (r, l, !fl)
    };
    if t == nl {
        let res = rec(n, nl, nr, nfl, &query[1..], memo);
        memo.insert((len, l, r), res);
        return res;
    }

    if nr < nl {
        nr += n;
    }

    if t < nl {
        t += n;
    }

    let mut res = usize::MAX;
    if nr == t {
        let d = nr - nl;
        res = res.min(rec(n, t % n, (t + 1) % n, nfl, &query[1..], memo) + d + 1);
        res = res.min(rec(n, t % n, (t + n - 1) % n, nfl, &query[1..], memo) + n - d + 1);
    } else if nr < t {
        let dl = t - nl;
        res = res.min(rec(n, t % n, (t + 1) % n, nfl, &query[1..], memo) + dl + (t - nr) + 1);
        res = res.min(rec(n, t % n, nr % n, nfl, &query[1..], memo) + n - dl);
    } else {
        let dl = t - nl;
        res = res.min(rec(n, t % n, nr % n, nfl, &query[1..], memo) + dl);
        res = res.min(rec(n, t % n, (t + n - 1) % n, nfl, &query[1..], memo) + n - dl + nr - t + 1);
    }

    memo.insert((len, l, r), res);
    res
}

fn main() {
    input! {n: usize, q: usize, query: [(char, usize); q]}
    println!("{}", rec(n, 0, 1, true, &query, &mut FxHashMap::default()))
}
