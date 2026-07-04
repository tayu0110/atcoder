use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut res = usize::MAX;
    let mut prev = vec![usize::MAX; 1000001];
    for (i, a) in a.into_iter().enumerate() {
        if prev[a] < usize::MAX {
            res = res.min(i - prev[a] + 1);
        }
        prev[a] = i;
    }

    println!("{}", res as i64)
}
