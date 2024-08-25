use proconio::*;

fn main() {
    input! {n: usize, q: usize}

    let _s = (0, 0);
    let _t = (0, n - 1);
    for _ in 0..q {
        input! {ty: u8}

        if ty == 1 {
            input! {_x: usize, _y: usize}
        } else if ty == 2 {
            input! {c: char}

            if c == 'A' {
            } 
        } 
    }
}
