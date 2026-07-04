use proconio::*;

fn main() {
    input! {n: usize, mut c: [usize; n - 1], mut a: [usize; n - 1]}

    c.insert(0, 0);
    a.insert(0, 1);
    let mut memo = vec![usize::MAX; n];
    memo[0] = 0;
    for i in 1..n {
        for j in i - c[i]..i {
            if a[j] == 1 {
                memo[i] = 1;
            } else {
                memo[i] = memo[i].min(memo[j] + 1);
            }
        }
    }

    println!(
        "{}",
        a.into_iter()
            .zip(memo)
            .skip(1)
            .filter_map(|v| (v.0 == 1).then_some(v.1))
            .sum::<usize>()
    )
}
