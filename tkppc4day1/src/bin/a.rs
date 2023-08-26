use proconio::*;

fn main() {
    input! {n: usize, _: usize}

    if n == 1 {
        println!("0")
    } else {
        println!("{}", n - 1);
    }
}
