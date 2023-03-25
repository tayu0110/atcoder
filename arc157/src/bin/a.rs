use proconio::*;

fn main() {
    input! {_: usize, a: usize, b: usize, c: usize, d: usize}

    if b == 0 && c == 0 && (a > 0 && d > 0) {
        println!("No");
        return;
    }

    if b + 1 == c || b == c || c + 1 == b {
        println!("Yes")
    } else {
        println!("No")
    }
}
