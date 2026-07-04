use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        (x, y) = (y, x)
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    let g = gcd(x, y);
    (x / g).saturating_mul(y)
}

fn main() {
    input! {n: usize, m: usize, y: usize, a: [usize; n]}

    let mut com = [0; 21];
    for i in m..=n {
        let mut c = 1u128;
        let mut d = 1 as u128;
        for j in 0..m {
            c *= (i - j) as u128;
            while d <= m as u128 && c % d == 0 {
                c /= d;
                d += 1;
            }
        }
        com[i] = c;
    }

    let mut res = 0u128;
    for i in 0u32..1 << n {
        let popcnt = i.count_ones() as usize;
        if popcnt < m {
            continue;
        }

        let mut l = 1;
        for j in 0..n {
            if i & (1 << j) != 0 {
                l = lcm(l, a[j]);
            }
        }

        if l > y {
            continue;
        }

        if (popcnt - m) & 1 == 0 {
            res = res.wrapping_add((y / l) as u128 * com[popcnt]);
        } else {
            res = res.wrapping_sub((y / l) as u128 * com[popcnt]);
        }
    }

    println!("{res}")
}
