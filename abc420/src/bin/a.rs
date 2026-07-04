use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    let to = (x - 1 + y) % 12;
    println!("{}", to + 1);
}
