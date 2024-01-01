use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if a[i] * a[j] == a[k] {
                    res += 1;
                }
            }
        }
    }

    println!("{res}")
}
