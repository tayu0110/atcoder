use proconio::*;

fn main() {
    input! {a: usize, b: usize}
    for i in 0..10 {
        if i != a + b {
            println!("{i}");
            return;
        }
    }
}
