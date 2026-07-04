use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0usize;
    for j in 1..n - 1 {
        let mut p = 0;
        for i in 0..j {
            p += (a[i] < a[j]) as usize;
        }
        let mut q = 0;
        for k in j + 1..n {
            q += (a[j] > a[k]) as usize;
        }

        res += p * q;
    }

    println!("{}", res)
}
