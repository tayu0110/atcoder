use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let sum = a.iter().sum::<usize>();
    for a in a {
        if sum - a == m {
            println!("Yes");
            return;
        }
    }
    println!("No")
}
