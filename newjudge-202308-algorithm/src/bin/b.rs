use proconio::*;

fn main() {
    input! {mut x: usize, k: usize}

    let mut ten = 1;
    for _ in 0..k {
        let mut y = x / ten;
        if y % 10 < 5 {
            y -= y % 10;
        } else {
            y += 10 - y % 10;
        }

        x = y * ten;
        ten *= 10;
    }

    println!("{}", x)
}
