fn main() {
    proconio::input! {n: usize, a: usize, b: usize, x: [usize; n]}

    println!(
        "{}",
        x.windows(2)
            .fold(0, |s, v| s + std::cmp::min((v[1] - v[0]) * a, b))
    )
}
