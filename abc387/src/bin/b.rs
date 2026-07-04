use proconio::*;

fn main() {
    input! {x: usize}

    let mut res = 0;
    for i in 1..=9 {
        for j in 1..=9 {
            if i * j != x {
                res += i * j;
            }
        }
    }
    println!("{res}")
}
