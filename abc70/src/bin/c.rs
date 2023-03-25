use proconio::input;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x / gcd(x, y) * y
}

fn main() {
    input! {n: usize, t: [usize; n]}
    println!("{}", t.into_iter().fold(1, |s, v| lcm(s, v)))
}
