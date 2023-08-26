use proconio::input;

fn enumerate(n: usize, ps: Vec<usize>) -> Vec<usize> {
    let mut res = vec![1];
    for p in ps {
        let mut now = p;
        let len = res.len();
        while now <= n {
            for i in 0..len {
                let k = res[i] as u128 * now as u128;
                if k <= n as u128 {
                    res.push(k as usize);
                }
            }
            now *= p;
        }

        res.sort();
    }
    res
}

fn main() {
    input! {n: usize, p: usize}

    let primes = (2..=p)
        .filter(|&i| (2..i).all(|j| i % j != 0))
        .collect::<Vec<_>>();

    if primes.len() < 5 {
        let res = enumerate(n, primes);
        println!("{}", res.len());
        return;
    }

    let mut xs = vec![];
    let mut ys = vec![];
    let (mut x, mut y) = (1, 1);
    for p in primes {
        if x * p as u128 <= y {
            x *= p as u128;
            xs.push(p);
        } else {
            y *= p as u128;
            ys.push(p);
        }
    }

    let xs = enumerate(n, xs);
    let ys = enumerate(n, ys);

    let mut res = 0;
    for x in xs {
        let (mut l, mut r) = (0, ys.len());
        while r - l > 1 {
            let m = (r + l) / 2;
            if x as u128 * ys[m] as u128 <= n as u128 {
                l = m;
            } else {
                r = m;
            }
        }

        res += l + 1;
    }

    println!("{}", res)
}
