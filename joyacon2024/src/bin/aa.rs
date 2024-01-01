use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let red = s.into_iter().filter(|&c| c == b'R').count();
    let blue = n - red;

    if red > blue {
        println!("Yes")
    } else {
        println!("No")
    }
}
