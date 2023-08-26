use proconio::*;

fn main() {
    input! {n: usize, m: usize, v: usize}

    let mut res = 0;
    for h in 1..=n {
        for w in 1..=m {
            let hw = h * w;
            let p = w - 1 + m * h - m;
            if hw % 2 == 1 && p % 2 == 1 {
                continue;
            }

            if hw * p / 2 > v {
                continue;
            }

            let t = v - hw * p / 2;
            if t % hw != 0 {
                continue;
            }

            let r = t / hw;

            let (i, j) = ((r - 1) / m + 1, (r - 1) % m + 1);
            if i + h - 1 <= n && j + w - 1 <= m {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
