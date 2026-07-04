use proconio::*;

const MAX: usize = 2000_000;

fn main() {
    input! {n: usize}

    let mut sieve = vec![0; MAX + 1];
    for i in 2..=MAX {
        if sieve[i] == 0 {
            for j in (1..).take_while(|j| i * j <= MAX) {
                sieve[i * j] = i;
            }
        }
    }

    let mut res = 0;
    for i in (1..).take_while(|&i| i * i <= n) {
        let mut factor = vec![];
        let mut now = i;
        while now > 1 {
            factor.push(sieve[now]);
            now /= sieve[now];
        }
        factor.sort_unstable();
        let mut rle = vec![];
        for f in factor {
            match rle.last_mut() {
                Some((p, cnt)) if *p == f => {
                    *cnt += 2;
                }
                _ => rle.push((f, 2)),
            }
        }

        if rle.into_iter().map(|r| r.1 + 1).product::<usize>() == 9 {
            res += 1;
        }
    }

    println!("{res}")
}
