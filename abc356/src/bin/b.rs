use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; m]}

    let mut b = vec![0; m];
    for _ in 0..n {
        input! {x: [usize; m]}
        for i in 0..m {
            b[i] += x[i];
        }
    }

    if a.into_iter().zip(b).all(|(a, b)| a <= b) {
        println!("Yes")
    } else {
        println!("No")
    }
}
