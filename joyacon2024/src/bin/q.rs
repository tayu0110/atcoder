use proconio::*;

fn main() {
    input! {h: usize, w: usize}

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if i + 1 < h {
                res += 1;
            }
            if j + 1 < w {
                res += 1;
            }
        }
    }

    println!("{res}")
}
