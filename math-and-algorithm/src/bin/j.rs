use proconio::input;

fn main() {
    input! {n: u128}
    println!("{}", (1..=n).fold(1, |s, v| s * v))
}
