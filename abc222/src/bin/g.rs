use proconio::input;

fn pow_mod(a: usize, n: usize, p: usize) -> usize {
    if n == 0 {
        1
    } else if n == 1 {
        a % p
    } else {
        let mut res = pow_mod(a, n / 2, p);
        res *= res;
        res %= p;
        if n % 2 == 1 {
            res *= a;
            res %= p;
        }
        res
    }
}

fn solve(mut n: usize) -> Option<usize> {
    if n % 2 == 0 {
        n /= 2;
    }
    n *= 9;
    let mut m = n;

    let mut phi = m;
    for i in (2..=n).take_while(|i| *i * *i <= n) {
        if m % i == 0 {
            while m % i == 0 {
                m /= i;
            }
            phi -= phi / i;
        }
    }
    if m != 1 {
        phi -= phi / m;
    }

    let mut d = vec![];
    for i in (1..=phi).take_while(|i| *i * *i <= phi) {
        if phi % i == 0 {
            d.push(i);
            if i * i != phi {
                d.push(phi / i);
            }
        }
    }

    d.sort();

    d.into_iter().find(|&nd| pow_mod(10, nd, n) == 1)
}

fn main() {
    input! {t: usize, k: [usize; t]}

    for n in k {
        if let Some(n) = solve(n) {
            println!("{}", n);
        } else {
            println!("-1");
        }
    }
}
