use proconio::*;

fn main() {
    input! {x: usize}

    let mut res = 0;
    for i in 0..x {
        if i % 2 == 0 {
            res += 3;
        } else {
            res -= 2;
        }
    }
    println!("{res}")
}
