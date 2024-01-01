use proconio::*;

#[fastout]
fn main() {
    input! {n: usize, mut a: [[u8; n]; n], q: usize}

    for _ in 0..q {
        input! {ty: u8, x: usize, y: usize}

        if ty == 1 {
            a.swap(x - 1, y - 1);
        } else {
            println!("{}", a[x - 1][y - 1])
        }
    }
}
