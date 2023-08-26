use proconio::*;

fn main() {
    input! {c: char}
    if c.is_uppercase() {
        println!("A")
    } else {
        println!("a")
    }
}
