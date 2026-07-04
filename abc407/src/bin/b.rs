use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    let mut sum = 0;
    for a in 1..=6 {
        for b in 1..=6 {
            if a + b >= x || a.abs_diff(b) >= y {
                sum += 1;
            }
        }
    }

    println!("{:.15}", sum as f64 / 36.);
}
