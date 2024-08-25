use proconio::*;

const R: [&str; 4] = ["", "maspy", "square1001", "PCTprobability"];

fn main() {
    input! {n: u8}
    println!("{}", R[n as usize])
}
