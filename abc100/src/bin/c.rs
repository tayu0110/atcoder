fn main() {
    proconio::input! {a: [i32]}
    println!("{}", a.iter().fold(0, |s, v| s + v.trailing_zeros()));
}
