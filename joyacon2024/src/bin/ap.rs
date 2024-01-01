use proconio::*;

fn main() {
    input! {w: usize, h: usize}

    if w * 3 == h * 4 {
        println!("4:3")
    } else {
        println!("16:9")
    }
}
