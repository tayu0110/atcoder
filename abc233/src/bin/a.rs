use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    if x >= y {
        println!("0")
    } else {
        println!("{}", (y - x + 9) / 10)
    }
}
