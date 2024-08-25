use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut t = vec![0; 256];
    for c in s {
        t[c as usize] += 1;
    }

    let mut k = vec![0; 300];
    for t in t {
        k[t] += 1;
    }

    if k.into_iter().skip(1).all(|c| c == 0 || c == 2) {
        println!("Yes")
    } else {
        println!("No")
    }
}
