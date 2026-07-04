use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], b: [usize; n]}

    let mut t = vec![0; n];
    for (i, b) in b.into_iter().enumerate() {
        t[b - 1] = i + 1;
    }
    if a.into_iter().zip(t).all(|(a, b)| a == b) {
        println!("Yes")
    } else {
        println!("No");
    }
}
