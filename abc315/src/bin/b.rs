use proconio::*;

fn main() {
    input! {m: usize, d: [usize; m]}

    let sum = d.iter().sum::<usize>();
    let mut mid = sum / 2;
    for (i, d) in d.into_iter().enumerate() {
        if d <= mid {
            mid -= d;
        } else {
            println!("{} {}", i + 1, mid + 1);
            return;
        }
    }
}
