use proconio::*;

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut memo = vec![0; n + 1];
    for i in 0..n {
        memo[i + 1] = memo[i] + a[i];
    }

    let &min = memo.iter().min().unwrap();
    memo.iter_mut().for_each(|a| *a -= min);
    // eprintln!("{memo:?}");
    println!("{}", memo[n])
}
