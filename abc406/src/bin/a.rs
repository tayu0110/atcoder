use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}

    if a > c {
        println!("Yes")
    } else if a == c && b >= d {
        println!("Yes")
    } else {
        println!("No")
    }
}
