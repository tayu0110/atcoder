use proconio::*;

fn main() {
    input! {n: usize}

    let (mut l, mut r) = (0, 2000000010);
    while r - l > 1 {
        let m = (r + l) / 2;
        if m * (m + 1) / 2 <= n {
            l = m;
        } else {
            r = m;
        }
    }

    if l * (l + 1) / 2 == n {
        println!("{l}")
    } else {
        println!("-1")
    }
}
