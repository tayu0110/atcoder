use proconio::*;

fn main() {
    input! {n: usize, l: [usize; n]}

    let mut now0 = 0;
    for i in 0..n {
        if l[i] == 0 {
            now0 += 1;
        } else {
            break;
        }
    }

    let mut now1 = n;
    for i in (0..n).rev() {
        if l[i] == 0 {
            now1 -= 1;
        } else {
            break;
        }
    }

    println!("{}", now1.saturating_sub(now0).saturating_sub(1))
}
