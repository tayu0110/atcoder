use proconio::*;

const T: usize = 200000;
const V: usize = 1000;

fn rec(
    nt: usize,
    nv: usize,
    ti: usize,
    t: &[usize],
    v: &[usize],
    memo: &mut Vec<Vec<Option<bool>>>,
) -> bool {
    let ti = if nt == t[ti] { ti + 1 } else { ti };
    if ti == t.len() {
        memo[nt][nv] = Some(nv == 0);
        return nv == 0;
    }
    if memo[nt][nv].is_some() {
        return memo[nt][nv].unwrap();
    }
    if nv > v[ti] {
        memo[nt][nv] = Some(false);
        return false;
    }
    for next in (nv.saturating_sub(1)..=(nv + 1).min(v[ti]).min(V)).rev() {
        let r = rec(nt + 1, next, ti, t, v, memo);
        if r {
            memo[nt][nv] = Some(r);
            return r;
        }
    }

    memo[nt][nv] = Some(false);
    false
}

fn solve() {
    input! {n: usize, mut t: [usize; n], mut v: [usize; n]}

    for i in 0..n {
        t[i] *= 10;
    }
    for i in 0..n - 1 {
        t[i + 1] += t[i];
    }
    for i in 0..n {
        v[i] *= 10;
    }

    let mut memo = vec![vec![None; V + 10]; T + 10];
    rec(0, 0, 0, &t, &v, &mut memo);

    let mut a = vec![];
    for i in 0..=t[n - 1] {
        for j in (0..=V).rev() {
            if memo[i][j].is_some() && memo[i][j].unwrap() {
                a.push(j);
                break;
            }
        }
    }

    let mut res = 0.0;
    for v in a.windows(2) {
        if v[0] == v[1] {
            res += v[0] as f64;
        } else {
            let diff = v[0].max(v[1]) - v[0].min(v[1]);
            res += v[0].min(v[1]) as f64 + diff as f64 / 2.0;
        }
    }

    println!("{}", res / 100.0);
}

fn main() {
    std::thread::Builder::new()
        .stack_size(1024 * 1024 * 256)
        .spawn(solve)
        .unwrap()
        .join()
        .unwrap();
}
