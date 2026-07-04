use cpio::*;

fn solve(s: usize, t: usize, u: &[Vec<usize>]) -> usize {
    if u.is_empty() {
        return s.abs_diff(t);
    }

    let pos = u[0].partition_point(|&u| u < s);
    let mut res = usize::MAX;
    if pos < u[0].len() {
        res = res.min(s.abs_diff(u[0][pos]) + solve(u[0][pos], t, &u[1..]));
    }
    if pos > 0 && !(pos < u[0].len() && s == u[0][pos]) {
        res = res.min(s.abs_diff(u[0][pos - 1]) + solve(u[0][pos - 1], t, &u[1..]));
    }
    res
}

fn main() {
    scan! {n: usize, p: [(usize, char); n], q: usize, query: [(usize, usize); q]}

    let mut u = vec![vec![]; 4];
    for (a, c) in p {
        match c {
            'J' => u[0].push(a),
            'O' => u[1].push(a),
            'I' => u[2].push(a),
            'G' => u[3].push(a),
            _ => {}
        }
    }

    for u in u.iter_mut() {
        u.sort_unstable();
    }

    for (s, t) in query {
        putln!(solve(s, t, &u));
    }
}
