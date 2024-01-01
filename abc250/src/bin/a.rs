use proconio::*;

fn main() {
    input! {h: usize, w: usize, r: usize, c: usize}

    let mut res = 4;
    if h == 1 {
        res -= 2;
    } else if r == 1 || r == h {
        res -= 1;
    }
    if w == 1 {
        res -= 2;
    } else if c == 1 || c == w {
        res -= 1;
    }

    println!("{res}")
}
