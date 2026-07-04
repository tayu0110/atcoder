use proconio::*;

fn main() {
    input! {a: usize, b: usize}
    if a * 3 > b * 2 {
        println!("Yes")
    } else {
        println!("No")
    }
}
