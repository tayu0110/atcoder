use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut score = vec![0i32; n];
    let mut last = vec![-1i32; n];
    for _ in 0..m {
        input! {ty: u8}

        if ty == 1 {
            input! {x: usize, y: usize}
            last[y - 1] = (x - 1) as i32;
        } else {
            input! {z: usize}
            score[z - 1] -= 1;
            if last[z - 1] >= 0 {
                score[last[z - 1] as usize] += 1;
                last[z - 1] = -1;
            }
        }
    }

    println!("{}", score.iter().join(" "))
}
