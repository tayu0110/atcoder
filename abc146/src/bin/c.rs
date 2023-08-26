use proconio::*;

const MAX: usize = 1000_000_000;

fn main() {
    input! {a: usize, b: usize, x: usize}

    let (mut l, mut r) = (0, MAX + 1);
    while r - l > 1 {
        let n = (r + l) / 2;
        if a * n + b * n.to_string().len() <= x {
            l = n;
        } else {
            r = n;
        }
    }
    println!("{}", l)
}
