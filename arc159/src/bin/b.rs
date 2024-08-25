use math::divisors_enumeration;
use proconio::*;
// use rand::Rng;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn solve(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return 1;
    }
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }

    let mut res = 0;
    for _ in 1.. {
        let g = gcd(a, b);
        a /= g;
        b /= g;

        let v = divisors_enumeration((b - a) as u64);
        let mut k = std::usize::MAX;
        for &v in &v {
            let v = v as usize;
            if v == 1 {
                continue;
            }
            if a % v == b % v {
                let t = a % v;
                k = k.min(t);
            }
        }

        if k == std::usize::MAX {
            k = a;
        }
        a -= k;
        b -= k;
        res += k;

        if a == 0 || b == 0 {
            return res;
        }
    }

    0
}

fn main() {
    input! {a: usize, b: usize}
    // use rand::thread_rng;
    // let mut rng = thread_rng();
    // let a: usize = rng.gen_range(1, 1000_000_000_000);
    // let b: usize = rng.gen_range(1, 1000_000_000_000);
    // eprintln!("a: {}, b: {}", a, b);

    println!("{}", solve(a, b));
}
