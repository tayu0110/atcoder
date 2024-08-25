use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut x: [usize; m]}

    let mut res = x.windows(2).map(|v| v[0].abs_diff(v[1])).sum::<usize>();

    let mut t = vec![vec![]; n + 1];
    for i in 0..m {
        t[x[i]].push(i);
    }

    let mut now = res;
    for i in 1..=n {
        for &nt in &t[i] {
            x[nt] = i + n;

            if nt > 0 {
                let prev = x[nt - 1];
                now += prev.abs_diff(i + n);
                now -= prev.abs_diff(i);
            }
            if nt + 1 < m {
                let next = x[nt + 1];
                now += next.abs_diff(i + n);
                now -= next.abs_diff(i);
            }
        }

        res = res.min(now);
    }

    println!("{res}");
}
