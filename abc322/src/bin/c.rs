use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    for i in 1..=n {
        let (mut l, mut r) = (-1, m as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if i <= a[m as usize] {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{}", a[r as usize] - i);
    }
}
