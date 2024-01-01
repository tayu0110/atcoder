use proconio::*;

fn main() {
    input! {n: usize, k: usize, p: [(usize, usize); n]}

    let mut res = 0;
    for x in 0..=100 {
        for y in 0..=100 {
            let mut cnt = 0;
            for &(a, b) in &p {
                if x <= a && a <= x + k && y <= b && b <= y + k {
                    cnt += 1;
                }
            }

            res = res.max(cnt);
        }
    }

    println!("{res}")
}
