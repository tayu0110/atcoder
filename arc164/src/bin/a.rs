use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {mut n: usize, k: usize}

        let mut res = vec![];
        while n >= 3 {
            let mut now = 1;
            while now * 3 <= n {
                now *= 3;
            }
            res.push(now);
            n -= now;
        }
        if n != 0 {
            res.resize(res.len() + n, 1);
        }

        if res.len() > k {
            println!("No");
            continue;
        }

        res.sort();
        if (k - res.len()) % 2 == 0 {
            println!("Yes");
        } else {
            println!("No")
        }
    }
}
