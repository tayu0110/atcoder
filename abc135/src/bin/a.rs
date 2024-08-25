use proconio::*;

fn main() {
    input! {a: u32, b: u32}

    if (a + b) % 2 == 0 {
        println!("{}", (a + b) / 2)
    } else {
        println!("IMPOSSIBLE")
    }
}
