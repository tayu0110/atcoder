use proconio::*;

fn main() {
    input! {n: usize, mut a: [usize; n + 1], b: [usize; n]}

    let sum = a.iter().sum::<usize>();
    for i in 0..n {
        let d = b[i].saturating_sub(a[i]);
        a[i] = a[i].saturating_sub(b[i]);
        a[i + 1] = a[i + 1].saturating_sub(d);
    }

    println!("{}", sum - a.iter().sum::<usize>())
}
