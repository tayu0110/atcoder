use proconio::*;

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

fn lcm(x: usize, y: usize) -> usize {
    x * y / gcd(x, y)
}

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}

    let dc = b / c - (a - 1) / c;
    let dd = b / d - (a - 1) / d;
    let dcd = b / lcm(c, d) - (a - 1) / lcm(c, d);
    println!("{}", (b - a + 1) + dcd - dc - dd)
}
