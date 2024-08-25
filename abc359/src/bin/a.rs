use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    println!("{}", s.into_iter().filter(|s| s == "Takahashi").count())
}
