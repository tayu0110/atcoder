use proconio::*;

fn main() {
    input! {n: u8, s: u8, t: u8}
    if (n ^ s ^ t) & 1 == 1 {
        println!("Alice")
    } else {
        println!("Bob")
    }
}
