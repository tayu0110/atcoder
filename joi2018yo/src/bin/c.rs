use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [[usize; m]; n]}

    let mut res = usize::MAX;
    for i in 0..n {
        for j in 0..m {
            let mut sum = 0;
            for k in 0..n {
                for l in 0..m {
                    sum += a[k][l] * i.abs_diff(k).min(j.abs_diff(l));
                }
            }

            res = res.min(sum);
        }
    }

    println!("{res}")
}
