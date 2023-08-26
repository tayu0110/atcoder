use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = vec![];
    for i in (1..=n).take_while(|&i| i * i <= n) {
        if n % i == 0 {
            for j in vec![i, n / i] {
                if j == 1 {
                    continue;
                }
                let mut now = n;
                while now % j == 0 {
                    now /= j;
                }
                if now == 1 {
                    res.push(j);
                } else if now % j == 1 {
                    res.push(j);
                }
            }
        }
    }
    for i in (1..=n).take_while(|&i| i * i <= n - 1) {
        if (n - 1) % i == 0 {
            if i != 1 {
                res.push(i);
            }
            if (n - 1) / i != 1 {
                res.push((n - 1) / i);
            }
        }
    }
    res.sort();
    res.dedup();
    println!("{}", res.len())
}
