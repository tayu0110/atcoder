use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    if a + b < 10 {
        println!("{}", a + b)
    } else {
        println!("error")
    }
}
