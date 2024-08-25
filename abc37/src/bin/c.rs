use proconio::*;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut sum = a.iter().take(k).sum::<usize>();
    let mut res = sum;
    for i in 0..n {
        if i + k >= n {
            break;
        }

        sum -= a[i];
        sum += a[i + k];
        res += sum;
    }

    println!("{}", res)
}
