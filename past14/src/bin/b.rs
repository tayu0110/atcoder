use proconio::*;

fn main() {
    input! {_: usize, a: String, b: String}

    let a = a.split('.').collect::<Vec<_>>();
    let b = b.split('.').collect::<Vec<_>>();

    let k = a[0].parse::<usize>().unwrap() + b[0].parse::<usize>().unwrap();
    let mut buf = vec![];
    let mut carry = 0;
    for (s, t) in a[1].bytes().rev().zip(b[1].bytes().rev()) {
        let s = s - b'0';
        let t = t - b'0';

        let u = s + t + carry;
        buf.push((u % 10 + b'0') as char);
        carry = u / 10;
    }

    println!(
        "{}.{}",
        k + carry as usize,
        buf.iter().rev().collect::<String>()
    )
}
