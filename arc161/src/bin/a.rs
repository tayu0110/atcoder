use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();

    let (a, b) = a.split_at(n / 2 + 1);
    for i in 0..b.len() {
        if a[i] < b[i] && b[i] > a[i + 1] {
            continue;
        }

        println!("No");
        return;
    }

    println!("Yes")
}
