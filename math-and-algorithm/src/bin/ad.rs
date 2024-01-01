use proconio::*;

fn main() {
    input! {n: usize, x: usize}

    for _ in 0..n {
        input! {a: usize}

        if a == x {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
