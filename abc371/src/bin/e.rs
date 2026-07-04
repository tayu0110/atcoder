use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut t = vec![vec![0]; n + 1];
    for (i, a) in a.into_iter().enumerate() {
        t[a].push(i + 1);
    }

    let mut res = 0usize;
    for mut t in t {
        t.push(n + 1);

        res += n * (n - 1) / 2 + n;
        for v in t.windows(2) {
            let interval = v[1] - v[0] - 1;
            if interval > 0 {
                res -= interval * (interval - 1) / 2 + interval;
            }
        }
    }

    println!("{res}")
}
