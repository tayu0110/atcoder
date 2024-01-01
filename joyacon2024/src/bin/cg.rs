use proconio::*;

fn main() {
    input! {m: usize, d: usize}

    if m % d == 0 {
        println!("YES")
    } else {
        println!("NO")
    }
}
