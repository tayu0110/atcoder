use proconio::input;

const MOD: u128 = 998244353;

fn totient(mut n: u128) -> u128 {
    let m = n;
    let mut res = n;
    for i in (2..).take_while(|&i| i * i <= m) {
        if n % i == 0 {
            res *= i - 1;
            res /= i;
            while n % i == 0 {
                n /= i;
            }
        }
    }

    if n > 1 {
        res *= n - 1;
        res /= n;
    }

    res
}

fn main() {
    input! {p: u128}

    let p = p - 1;
    let mut res = 0;
    for i in (1..).take_while(|&i| i * i <= p) {
        if p % i == 0 {
            res += i * totient(i);
            res %= MOD;
            if i != p / i {
                let j = p / i;
                res += j * totient(j);
                res %= MOD;
            }
        }
    }

    println!("{}", (res + 1) % MOD);
}
