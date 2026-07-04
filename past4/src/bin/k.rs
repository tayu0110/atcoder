use proconio::*;

const M: usize = 1000_000_000;

fn main() {
    input! {k: usize}

    let mut t = vec![];
    for _ in 0..k {
        input! {n: usize, a: [usize; n]}
        let mut cnt = [0; 21];
        let mut b = 0usize;
        for a in a {
            b += cnt[a + 1..].iter().sum::<usize>();
            cnt[a] += 1;
        }
        b %= M;
        t.push((b, cnt));
    }

    input! {q: usize, b: [usize; q]}
    let mut res = 0usize;
    let mut cnt = [0; 21];
    let mut cum = [0; 21];
    for b in b {
        res += t[b - 1].0;
        res %= M;

        for (i, &c) in t[b - 1].1.iter().enumerate() {
            res += c * cum[i];
            res %= M;
            cnt[i] += c;
        }

        for i in (1..21).rev() {
            cum[i - 1] = cum[i] + cnt[i];
            cum[i - 1] %= M;
        }
    }

    println!("{res}");
}
