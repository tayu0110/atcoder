use proconio::*;

fn main() {
    input! {a: usize, b: usize}

    for i in 0..100000 {
        let na = i * 108 / 100;
        let nb = i * 110 / 100;

        if a == na - i && b == nb - i {
            println!("{i}");
            return;
        }
    }

    println!("-1");
}
