use proconio::input;

fn main() {
    input! {n: usize, a: [usize; 2*n-1]}

    println!("{}", a.into_iter().fold(0, |s, v| s ^ v));
}
