use proconio::*;

fn main() {
    input! {n: usize, w: usize, a: [usize; n]}

    let mut res = vec![false; w + 1];
    for i in 0..n {
        if a[i] <= w {
            res[a[i]] = true;
        }
        for j in i + 1..n {
            if a[i] + a[j] <= w {
                res[a[i] + a[j]] = true;
            }
            for k in j + 1..n {
                let sum = a[i] + a[j] + a[k];
                if sum <= w {
                    res[sum] = true;
                }
            }
        }
    }

    println!("{}", res.into_iter().filter(|&f| f).count())
}
