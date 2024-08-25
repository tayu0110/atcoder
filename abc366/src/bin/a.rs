use proconio::*;

fn main() {
    input! {n: usize, t: usize, a: usize}

    if t * 2 > n || a * 2 > n {
        println!("Yes")
    } else {
        println!("No")
    }
}
