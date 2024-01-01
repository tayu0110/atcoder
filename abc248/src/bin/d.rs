use proconio::*;

fn lower(tar: usize, a: &[usize]) -> usize {
    let (mut l, mut r) = (0, a.len());
    while r - l > 1 {
        let m = (r + l) / 2;
        if a[m] <= tar {
            l = m;
        } else {
            r = m;
        }
    }
    l
}

fn main() {
    input! {n: usize, a: [usize; n], q: usize, query: [(usize, usize, usize); q]}

    let mut t = vec![vec![0]; n + 1];
    for (i, a) in a.into_iter().enumerate() {
        t[a].push(i + 1);
    }

    for (l, r, x) in query {
        let v = &t[x];
        let ri = lower(r, v);
        let mut li = lower(l, v);
        if v[li] < l {
            li += 1;
        }

        println!("{}", ri + 1 - li);
    }
}
