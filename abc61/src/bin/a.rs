use proconio::*;

fn main() {
    input! {a: i32, b: i32, c: i32}

    if (a..=b).contains(&c) {
        println!("Yes")
    } else {
        println!("No")
    }
}
