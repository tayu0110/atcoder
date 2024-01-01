use proconio::*;

fn main() {
    input! {n: usize}

    if n == 1 {
        println!("BOWWOW");
        return;
    }

    let n = n * (n + 1) / 2;
    for i in 2..n {
        if n % i == 0 {
            println!("BOWWOW");
            return;
        }
    }

    println!("WANWAN");
}
