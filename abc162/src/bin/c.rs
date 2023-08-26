use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn main() {
    input! {k: usize}

    let mut res = 0;
    for i in 1..=k {
        for j in 1..=k {
            for k in 1..=k {
                res += gcd(i, gcd(j, k));
            }
        }
    }
    println!("{}", res)
}
