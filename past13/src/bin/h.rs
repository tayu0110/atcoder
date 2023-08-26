use proconio::*;

fn main() {
    input! {_: usize, d: marker::Bytes}

    let mut v = vec![0usize; 10];
    for c in d {
        v[(c - b'0') as usize] += 1;
    }

    let mut res = 0;
    for i in 0..10 {
        for j in i + 1..10 {
            res += (j - i) * v[i] * v[j];
        }
    }

    println!("{}", res)
}
