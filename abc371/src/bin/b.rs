use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, char); m]}

    let mut memo = vec![false; n];
    for (a, b) in p {
        if b == 'M' && !memo[a - 1] {
            println!("Yes");
            memo[a - 1] = true;
        } else {
            println!("No")
        }
    }
}
