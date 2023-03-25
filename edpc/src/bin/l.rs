use proconio::input;

fn rec(l: usize, r: usize, a: &[i64], memo: &mut Vec<Vec<i64>>) -> i64 {
    if l == r {
        return 0;
    }

    if memo[l][r] != std::i64::MIN {
        return memo[l][r];
    }

    let mut res = {
        let res = rec(l + 1, r, &a[1..], memo);
        a[0] - res
    };
    res = std::cmp::max(res, {
        let res = rec(l, r - 1, &a[..a.len() - 1], memo);
        a.last().unwrap() - res
    });

    memo[l][r] = res;
    res
}

fn main() {
    input! {n: usize, a: [i64; n]}

    let mut memo = vec![vec![std::i64::MIN; n + 1]; n];
    println!("{}", rec(0, n, &a, &mut memo));
}
