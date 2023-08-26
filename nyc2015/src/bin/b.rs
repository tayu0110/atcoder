use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n]}
    a.sort();

    let mut sum = 0;
    let mut res = 0;
    for i in 0..n {
        if sum >= a[i] {
            continue;
        }

        sum += a[i];
        res += 1;
    }

    println!("{}", res);
}
