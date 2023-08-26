use proconio::*;

fn main() {
    input! {n: usize, s: marker::Chars}

    if n % 2 == 1 {
        println!("No");
        return;
    }

    let m = n / 2;
    if s[..m] == s[m..] {
        println!("Yes")
    } else {
        println!("No")
    }
}
