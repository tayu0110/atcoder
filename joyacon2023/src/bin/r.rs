use proconio::input;

fn main() {
    input! {n: usize, x: usize, p: [usize; n]}

    println!("{}", p.into_iter().position(|v| v == x).unwrap() + 1);
}
