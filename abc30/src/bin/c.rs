use proconio::*;

fn main() {
    input! {n: usize, m: usize, x: usize, y: usize, a: [usize; n], b: [usize; m]}

    let mut now = 0;
    for i in 0.. {
        for (x, a) in [(x, &a), (y, &b)] {
            let p = a.partition_point(|&a| a < now);
            if p == a.len() {
                println!("{i}");
                return;
            }

            now = a[p] + x;
        }
    }
}
