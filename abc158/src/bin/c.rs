use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    for i in 1..2000 {
        let na = i * 108 / 100 - i;
        let nb = i * 110 / 100 - i;
        if na == a && nb == b {
            println!("{}", i);
            return;
        }
    }

    println!("-1")
}
