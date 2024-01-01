use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n]}

    let mut diff = 0;
    let mut max = 0;
    for &a in &a {
        if a == 1 {
            diff += 1;
        } else {
            diff -= 1;
        }
        max = max.max(diff);
    }

    println!("{}", a.iter().sum::<i32>() - max)
}
