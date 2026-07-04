use proconio::*;

fn main() {
    input! {n: usize}

    let mut a = vec![vec![]];
    for i in 1..=n {
        input! {mut k: [usize; i]}
        k.insert(0, 0);
        a.push(k);
    }

    let mut now = 1;
    for i in 1..=n {
        if now >= i {
            now = a[now][i];
        } else {
            now = a[i][now];
        }
    }

    println!("{now}")
}
