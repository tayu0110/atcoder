use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    let c = a * b;
    if c % 2 == 0 {
        println!("Even")
    } else {
        println!("Odd")
    }
}
