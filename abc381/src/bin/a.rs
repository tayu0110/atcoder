use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    if n % 2 != 1 {
        println!("No");
        return;
    }

    if s[..n / 2].iter().all(|&b| b == b'1')
        && s[n / 2] == b'/'
        && s[n / 2 + 1..].iter().all(|&b| b == b'2')
    {
        println!("Yes")
    } else {
        println!("No")
    }
}
