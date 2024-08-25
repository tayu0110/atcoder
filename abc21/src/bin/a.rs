use proconio::*;

fn main() {
    input! {n: u8}
    println!("{}", n.count_ones());
    for i in 0..4 {
        if n & (1 << i) != 0 {
            println!("{}", 1 << i)
        }
    }
}
