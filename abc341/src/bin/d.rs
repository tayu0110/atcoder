use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        (x, y) = (y, x);
    }

    x
}

fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn main() {
    input! {n: usize, m: usize, mut k: usize}

    let l = lcm(n, m);
    let cnt = l / n + l / m - 2;

    let mut t = k / cnt;
    if t * cnt == k {
        t -= 1;
    }
    k %= cnt;

    let base = t * l;
    eprintln!("base: {base}, cnt: {cnt}");
    let (mut s, mut t) = (1, 1);
    for i in 0..k {
        if s * n < t * m {
            if i == k - 1 {
                println!("{}", base + s * n);
                return;
            }
            s += 1;
        } else {
            if i == k - 1 {
                println!("{}", base + t * m);
                return;
            }
            t += 1;
        }
    }

    if s * n < t * m {
        println!("{}", base + s * n);
    } else {
        println!("{}", base + t * m);
    }
}
