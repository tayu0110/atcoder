use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, s: marker::Chars}

    let mut cross = vec![0usize; n + 1];
    for (i, &c) in s.iter().enumerate() {
        cross[i + 1] = cross[i];
        if c == 'x' {
            cross[i + 1] += 1;
        }
    }

    let mut res = 0;
    for i in 0..n {
        let (mut l, mut r) = (i, n * m);
        while r - l > 1 {
            let p = (r + l) / 2;
            let np = p / n;
            let c = if np > 0 {
                let nc = cross[n] - cross[i] + cross[p % n + 1];
                nc + (np - 1) * cross[n]
            } else {
                cross[p + 1] - cross[i]
            };

            if c <= k {
                l = p;
            } else {
                r = p;
            }
        }

        // eprintln!("i: {}, l: {}, tmp: {}, res: {}", i, l, r - i, res);
        res = res.max(r - i);
    }

    println!("{}", res);
}
