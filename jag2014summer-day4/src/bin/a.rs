use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}

    for t in 0..1000000 {
        let k = 60 * t + c;
        if k % (a + b) <= a {
            println!("{k}");
            return;
        }
    }

    println!("-1");
}
