use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    if x >= y {
        println!("{}", x - y);
        return;
    }
}
