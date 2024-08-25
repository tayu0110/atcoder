use proconio::*;

fn main() {
    input! {n: i32, l: i32}

    let t = (0..n).map(|v| v + l).collect::<Vec<_>>();
    let sum = t.iter().sum::<i32>();
    let mut diff = std::i32::MAX;
    let mut res = 0;
    for t in t.iter().take(n as usize) {
        if diff > (sum - (sum - t)).abs() {
            diff = (sum - (sum - t)).abs();
            res = sum - t;
        }
    }

    println!("{}", res)
}
