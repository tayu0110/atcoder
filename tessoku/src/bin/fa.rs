use proconio::*;

#[fastout]
fn main() {
    input! {d: usize, x: i64, mut a: [i64; d - 1], q: usize}

    let mut b = vec![0; d];
    b[0] = x;
    for i in 0..d - 1 {
        b[i + 1] = b[i] + a[i];
    }

    for _ in 0..q {
        input! {s: usize, t: usize}

        if b[s - 1] > b[t - 1] {
            println!("{}", s);
        } else if b[s - 1] < b[t - 1] {
            println!("{}", t);
        } else {
            println!("Same");
        }
    }
}
