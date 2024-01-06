use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = vec![];
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n {
                if i + j + k <= n {
                    res.push((i, j, k));
                }
            }
        }
    }
    res.sort();

    for (i, j, k) in res {
        println!("{i} {j} {k}");
    }
}
