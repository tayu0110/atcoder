use proconio::*;

fn main() {
    input! {x: usize}
    if (3..=18).contains(&x) {
        println!("Yes")
    } else {
        println!("No")
    }
}
