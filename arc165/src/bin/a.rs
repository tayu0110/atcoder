use math::miller_rabin_test;
use proconio::*;

fn gcd(mut x: u64, mut y: u64) -> u64 {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }
    x
}

fn main() {
    input! {t: usize}

    'B: for _ in 0..t {
        input! {n: u64}

        if miller_rabin_test(n) {
            println!("No");
            continue;
        }

        for i in (1..).take_while(|&i| i * i <= n) {
            if n % i == 0 {
                let j = n / i;
                if gcd(i, j) > 1 {
                    continue;
                }

                if i + j <= n {
                    println!("Yes");
                    continue 'B;
                }
            }
        }

        println!("No")
    }
}
