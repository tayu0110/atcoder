use proconio::*;

fn main() {
    input! {mut h: usize, mut w: usize, q: usize}

    for _ in 0..q {
        input! {ty: u8, r: usize}

        if ty == 1 {
            println!("{}", r * w);
            h -= r;
        } else {
            println!("{}", r * h);
            w -= r;
        }
    }
}
