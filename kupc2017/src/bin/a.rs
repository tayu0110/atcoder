use proconio::*;

fn main() {
    input! {n: usize, k: usize, mut a: [usize; n]}

    a.sort();
    let len = a.len();
    let mut sum = 0;
    for i in 0..len {
        sum += a.pop().unwrap();

        if sum >= k {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1")
}
