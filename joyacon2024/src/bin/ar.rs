use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}

    if a + b >= c {
        println!("Yes")
    } else {
        println!("No")
    }
}
