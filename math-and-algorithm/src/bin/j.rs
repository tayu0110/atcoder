use proconio::input;

fn main() {
    input! {n: u128}
    println!("{}", (1..=n).product::<u128>())
}
