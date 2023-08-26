use proconio::*;

fn main() {
    input! {mut n: usize}

    let mut buf = vec![];
    while n > 0 {
        n -= 1;
        buf.push(((n % 26) as u8 + b'a') as char);
        n /= 26;
    }

    buf.reverse();
    println!("{}", buf.into_iter().collect::<String>())
}
