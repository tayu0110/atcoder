use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    if x.max(y) - x.min(y) < 3 {
        println!("Yes")
    } else {
        println!("No")
    }
}
