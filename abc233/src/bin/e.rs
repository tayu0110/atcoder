use proconio::*;

fn main() {
    input! {mut x: marker::Bytes}

    let mut sum = x.iter().map(|&c| (c - b'0') as usize).sum::<usize>();
    let mut now = sum;
    let mut res = vec![];
    while let Some(c) = x.pop().map(|c| (c - b'0') as usize) {
        let d = now % 10;
        res.push((d as u8 + b'0') as char);
        now /= 10;
        sum -= c;
        now += sum;
    }

    while now > 0 {
        res.push(((now % 10) as u8 + b'0') as char);
        now /= 10;
    }

    res.reverse();
    println!("{}", res.iter().collect::<String>())
}
