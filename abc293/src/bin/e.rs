use num::integer::Roots;
use proconio::*;

fn pow_mod(a: u128, n: u128, p: u128) -> u128 {
    if n == 0 {
        1
    } else if n == 1 {
        a % p
    } else {
        let res = pow_mod(a, n / 2, p);
        if n % 2 == 1 {
            res * res % p * a % p
        } else {
            res * res % p
        }
    }
}

fn main() {
    input! {a: u128, x: u128, m: u128}

    if m == 1 {
        println!("0");
        return;
    }

    if x < 1000 {
        let mut res = 0;
        for i in 0..x {
            res += pow_mod(a, i, m);
            res %= m;
        }
        println!("{}", res);
        return;
    }

    let a = a % m;

    if a == 0 {
        println!("1");
        return;
    }

    let qx = x.sqrt();
    let t = {
        let mut sum = 0;
        let mut now = 1;
        for _ in 0..qx {
            sum += now;
            sum %= m;
            now *= a;
            now %= m;
        }
        sum
    };

    let mut res = 0;
    let mut na = 1;
    let mut now = 0;
    let qa = pow_mod(a, qx, m);
    while now < x {
        if now + qx <= x {
            res += t * na % m;
            res %= m;
        } else {
            let nx = x - now;
            let mut now = 1;
            let mut sum = 0;
            for _ in 0..nx {
                sum += now;
                sum %= m;
                now *= a;
                now %= m;
            }

            res += sum * na % m;
            res %= m;
        }

        na *= qa;
        na %= m;

        now += qx;
    }

    println!("{}", res)
}
