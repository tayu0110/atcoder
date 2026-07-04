use proconio::*;

fn main() {
    input! {_: usize, t: marker::Bytes, a: marker::Bytes}

    if t.into_iter()
        .zip(a)
        .filter(|&(t, a)| t == b'o' && a == b'o')
        .next()
        .is_some()
    {
        println!("Yes")
    } else {
        println!("No")
    }
}
