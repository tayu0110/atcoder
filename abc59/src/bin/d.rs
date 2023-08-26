use proconio::*;

fn main() {
    input! {x: i64, y: i64}

    if (x - y).abs() <= 1 {
        println!("Brown")
    } else {
        println!("Alice")
    }
}
