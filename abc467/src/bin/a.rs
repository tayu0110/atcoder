use proconio::*;

fn main() {
    input! {h: usize, w: usize}
    if w * 100 * 100 >= 25 * h * h {
        println!("Yes")
    } else {
        println!("No")
    }
}
