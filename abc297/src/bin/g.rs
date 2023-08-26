use proconio::input;

fn main() {
    input! {n: usize, l: usize, r: usize, a: [usize; n]}
    println!(
        "{}",
        if a.into_iter().fold(0, |s, v| s ^ (v % (l + r) / l)) > 0 {
            "First"
        } else {
            "Second"
        }
    )
}
