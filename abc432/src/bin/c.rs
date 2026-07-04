use proconio::*;

fn main() {
    input! {n: usize, x: usize, y: usize, mut a: [usize; n]}

    a.sort_unstable();
    let mut diff = vec![0; n];
    for i in 1..n {
        let num = (a[i] - a[0]) * x;
        let den = y - x;
        if num % den != 0 {
            println!("-1");
            return;
        }

        diff[i] = num / den;
    }
    if *diff.iter().max().unwrap() > a[0] {
        println!("-1");
        return;
    }

    let mut sum = a[0];
    for i in 1..n {
        sum += a[0] - diff[i];
    }
    println!("{sum}")
}
