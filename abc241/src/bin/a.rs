use proconio::*;

fn main() {
    input! {a: [usize; 10]}

    let mut now = 0;
    for _ in 0..3 {
        now = a[now];
    }

    println!("{now}")
}
