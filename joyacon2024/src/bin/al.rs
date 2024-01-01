use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize}

    if a.abs_diff(c) <= d || (a.abs_diff(b) <= d && b.abs_diff(c) <= d) {
        println!("Yes")
    } else {
        println!("No")
    }
}
