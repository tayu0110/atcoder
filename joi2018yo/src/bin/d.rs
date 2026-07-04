use proconio::*;

fn solve<const INNER: bool>(min: usize, max: usize, l: &[usize], memo: &mut [u8]) -> bool {
    if l.is_empty() {
        return true;
    }
    if memo[0] < 2 {
        return memo[0] != 0;
    }

    let mut sum = 0;
    for i in 0..l.len() + INNER as usize - 1 {
        sum += l[i];
        if sum < min {
            continue;
        }
        if sum > max {
            break;
        }
        if solve::<true>(min, max, &l[i + 1..], &mut memo[i + 1..]) {
            memo[0] = 1;
            return true;
        }
    }
    memo[0] = 0;
    false
}

fn main() {
    input! {n: usize, l: [usize; n]}

    let mut sum = vec![];
    for i in 0..n {
        for j in i + 1..=n {
            sum.push(l[i..j].iter().sum::<usize>());
        }
    }
    sum.sort_unstable();
    sum.dedup();

    let m = sum.len();
    let mut res = usize::MAX;
    for i in 0..m {
        for j in i..m {
            if sum[j] - sum[i] >= res {
                continue;
            }
            let mut memo = vec![2; n];
            if solve::<false>(sum[i], sum[j], &l, &mut memo) {
                res = res.min(sum[j] - sum[i]);
                break;
            }
        }
    }
    println!("{res}")
}
