use proconio::*;

fn main() {
    input! {mut a: usize, mut b: usize}

    if a == 1 {
        a += 13;
    }
    if b == 1 {
        b += 13;
    }

    if a > b {
        println!("Alice")
    } else if a == b {
        println!("Draw")
    } else {
        println!("Bob")
    }
}
