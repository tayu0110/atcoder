use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut t = vec![0; 200001];
    for a in a {
        t[a] += 1;
    }

    for i in 0..199995 {
        if t[i] > 0 && t[i + 3] > 0 && t[i + 6] > 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
