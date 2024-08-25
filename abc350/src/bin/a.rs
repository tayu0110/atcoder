use proconio::*;

fn main() {
    input! {s: String}

    let s: usize = s[3..].parse().unwrap();
    if 1 <= s && s < 350 && s != 316 {
        println!("Yes")
    } else {
        println!("No")
    }
}
