use proconio::*;

fn main() {
    input! {a: u8, b: u8}
    let s = (0..b).map(|_| (a + b'0') as char).collect::<String>();
    let t = (0..a).map(|_| (b + b'0') as char).collect::<String>();
    println!("{}", s.min(t));
}
