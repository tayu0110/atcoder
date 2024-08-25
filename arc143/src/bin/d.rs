#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn dfs(
    now: usize,
    a: &Vec<usize>,
    ck: &mut Vec<bool>,
    tmp: &mut Vec<bool>,
    res: &mut Vec<i32>,
    t: &Vec<Vec<(usize, usize)>>,
) {
    tmp[now] = true;

    for (to, idx) in &t[now] {
        if res[*idx] >= 0 {
            continue;
        }

        if a[*idx] == now + 1 {
            res[*idx] = 0;
        } else {
            res[*idx] = 1;
        }

        if ck[*to] || tmp[*to] {
            continue;
        }

        dfs(*to, a, ck, tmp, res, t);
    }

    ck[now] = true;
    tmp[now] = false;
}

fn main() {
    input! {n: usize, m: usize, a: [usize; m], b: [usize; m]};

    let mut t = vec![vec![]; n];
    for (i, (na, nb)) in a.iter().zip(b.iter()).enumerate() {
        let (na, nb) = (*na - 1, *nb - 1);

        t[na].push((nb, i));
        t[nb].push((na, i));
    }

    let mut tmp = vec![false; n];
    let mut ck = vec![false; n];
    let mut res = vec![-1; m];
    for start in 0..n {
        if ck[start] {
            continue;
        }
        dfs(start, &a, &mut ck, &mut tmp, &mut res, &t);
    }

    for v in res {
        print!("{}", v);
    }
    println!();
}
