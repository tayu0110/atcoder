use proconio::*;

fn main() {
    input! {n: usize}

    let mut a = 0;
    let mut t = 0;
    for _ in 0..n {
        input! {x: usize, y: usize}
        t += x;
        a += y;
    }

    if t > a {
        println!("Takahashi")
    } else if t < a {
        println!("Aoki")
    } else {
        println!("Draw")
    }
}
