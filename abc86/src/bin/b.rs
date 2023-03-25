use proconio::*;

fn main() {
    input! {mut a: String, b: String}
    a.push_str(&b);

    let c: usize = a.parse().unwrap();

    if (0..=c).any(|d| d * d == c) {
        println!("Yes")
    } else {
        println!("No")
    }
}
