use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    if x != 0 && y != 0 {
        println!("2")
    } else if x == 0 && y == 0 {
        println!("0")
    } else {
        println!("1")
    }
}
