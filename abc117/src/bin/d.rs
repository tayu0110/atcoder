use proconio::input;

fn solve(now: usize, k: usize, eq: bool, a: &[usize], memo: &mut Vec<Vec<Vec<usize>>>) -> usize {
    let max = if eq { (k & (1 << now)) >> now } else { 1 };
    let mut res = 0;
    for i in 0..=max {
        let f = i < (k & (1 << now)) >> now;
        let is_eq = !f && eq;
        if memo[is_eq as usize][i][now] == std::usize::MAX {
            let mut sum = 0;
            for &a in a {
                sum += (((a & (1 << now)) >> now) ^ i) << now;
            }

            memo[is_eq as usize][i][now] = sum;

            if now != 0 {
                memo[is_eq as usize][i][now] += solve(now - 1, k, is_eq, a, memo);
            }
        }

        res = std::cmp::max(memo[is_eq as usize][i][now], res);
    }

    res
}

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut memo = vec![vec![vec![std::usize::MAX; 60]; 2]; 2];
    println!("{}", solve(55, k, true, &a, &mut memo))
}
