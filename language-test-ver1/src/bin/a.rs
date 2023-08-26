use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}

    let mut t = vec![0; 4];
    for c in s {
        t[(c - b'1') as usize] += 1;
    }

    println!("{} {}", t.iter().max().unwrap(), t.iter().min().unwrap())
}
