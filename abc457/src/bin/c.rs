use proconio::*;

fn main() {
    input! {n: usize, mut k: usize}
    k -= 1;

    let mut a = vec![];
    for _ in 0..n {
        input! {l: usize, na: [usize; l]}
        a.push(na);
    }
    input! {c: [usize; n]}
    for i in 0..n {
        if k < a[i].len() * c[i] {
            println!("{}", a[i][k % a[i].len()]);
            break;
        }

        k -= a[i].len() * c[i];
    }
}
