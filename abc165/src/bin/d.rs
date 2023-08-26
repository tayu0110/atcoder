use proconio::*;

fn main() {
    input! {a: usize, b: usize, n: usize}

    let f = |x: usize| a * x / b - a * (x / b);
    println!("{}", f((n / b * b).saturating_sub(1)).max(f(n)));
}
