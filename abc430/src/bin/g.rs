use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {l: usize, r: usize, x: usize}
        } else if ty == 2 {
            input! {l: usize, r: usize, x: usize}
        } else {
            input! {l: usize, r: usize}
        }
    }
}
