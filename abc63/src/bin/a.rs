use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    if a + b > 9 {
        println!("error")
    } else {
        println!("{}", a + b)
    }
}
