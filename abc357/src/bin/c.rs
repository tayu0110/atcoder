use itertools::Itertools;
use proconio::*;

fn rec(k: usize, u: usize, d: usize, l: usize, r: usize, res: &mut [Vec<char>]) {
    if k == 0 {
        res[u][l] = '#';
        return;
    }

    let t = (d - u) / 3;
    for a in (u..d).step_by(t) {
        for b in (l..r).step_by(t) {
            if a == u + t && b == l + t {
                continue;
            }

            rec(k - 1, a, a + t, b, b + t, res);
        }
    }
}

fn main() {
    input! {n: usize}

    let d = 3usize.pow(n as u32);
    let mut res = vec![vec!['.'; d]; d];
    rec(n, 0, d, 0, d, &mut res);

    for r in res {
        println!("{}", r.iter().join(""));
    }
}
