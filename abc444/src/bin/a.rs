use proconio::*;

fn main() {
    input! {n: marker::Bytes}
    if n.iter().all(|&b| b == n[0]) {
        println!("Yes")
    } else {
        println!("No")
    }
}
