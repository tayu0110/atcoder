use proconio::*;

fn main() {
    input! {n: usize, a: usize, b: usize}

    for i in 0..a * n {
        for j in 0..b * n {
            if (i / a) % 2 != (j / b) % 2 {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}
