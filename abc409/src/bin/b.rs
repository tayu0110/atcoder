use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    println!(
        "{}",
        (0..=n)
            .filter_map(|t| {
                let mut cnt = 0;
                for i in 0..n {
                    if a[i] >= t {
                        cnt += 1;
                        if cnt >= t {
                            return Some(t);
                        }
                    }
                }
                None
            })
            .max()
            .unwrap()
    )
}
