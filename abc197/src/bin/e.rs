use proconio::*;

fn main() {
    input! {n: usize, p: [(i64, usize); n]}

    let mut t = vec![vec![]; n + 1];
    for (x, c) in p {
        t[c].push(x);
    }

    {
        let mut f = 0;
        let mut b = 0;
        while f < n {
            if t[f].is_empty() {
                while b < n && t[b].is_empty() {
                    b += 1;
                }

                if b < n {
                    t.swap(f, b);
                }
            }

            if t[f].is_empty() {
                break;
            }

            f += 1;
        }

        t.resize(f, vec![]);
    }
}
