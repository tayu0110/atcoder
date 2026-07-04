use proconio::*;

fn main() {
    input! {n: i32}

    let mut res = 0;
    for i in 1..=n {
        if i % 2 == 0 {
            res += i * i * i;
        } else {
            res -= i * i * i;
        }
    }
    println!("{res}")
}
