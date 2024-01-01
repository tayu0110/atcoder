use proconio::*;

fn main() {
    input! {a: usize, b: usize, n: usize}

    for i in n.. {
        if i % a == 0 && i % b == 0 {
            println!("{i}");
            return;
        }
    }
}
