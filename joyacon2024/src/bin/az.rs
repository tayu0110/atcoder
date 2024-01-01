use proconio::*;

fn main() {
    input! {a: i32, b: i32}

    let (a, b) = (a.abs(), b.abs());

    if a < b {
        println!("Ant");
    } else if a == b {
        println!("Draw");
    } else {
        println!("Bug");
    }
}
