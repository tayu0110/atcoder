use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut c = vec![0; n + 1];
    for a in a {
        c[a] += 1;
    }

    let mut d = vec![0; n + 1];
    for c in c.into_iter().skip(1) {
        d[c] += 1;
    }

    let mut s = vec![0; n + 2];
    let mut t = vec![0; n + 2];
    for i in 1..=n {
        s[i] += d[i];
        t[i] += i * d[i];
        s[i + 1] += s[i];
        t[i + 1] += t[i];
    }

    for k in 1..=n {
        let max = n / k;
        let mut res = 0;
        for x in 1..=max {
            let r = (t[x - 1] + (s[n] - s[x - 1]) * x) / x;
            if r >= k {
                res = x;
            }
        }

        println!("{res}")
    }
}
