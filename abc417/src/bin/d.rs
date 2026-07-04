use proconio::*;

fn dfs(
    now: usize,
    ten: usize,
    present: &[(usize, usize, usize)],
    cum: &[usize],
    memo: &mut [[usize; 1001]],
) -> usize {
    if present.is_empty() {
        memo[now][ten] = ten;
        return ten;
    }
    if memo[now][ten] < usize::MAX {
        return memo[now][ten];
    }

    let (p, a, b) = present[0];
    memo[now][ten] = if ten <= p {
        dfs(now + 1, ten + a, &present[1..], &cum[1..], memo)
    } else {
        dfs(
            now + 1,
            ten.saturating_sub(b),
            &present[1..],
            &cum[1..],
            memo,
        )
    };
    memo[now][ten]
}

fn main() {
    input! {n: usize, present: [(usize, usize, usize); n], q: usize, query: [usize; q]}

    let mut cum = vec![0; n + 1];
    for (i, p) in present.iter().enumerate() {
        cum[i + 1] = cum[i] + p.2;
    }
    let mut memo = vec![[usize::MAX; 1001]; n + 1];
    for now in 0..=n {
        for i in 0..1001 {
            dfs(now, i, &present[now..], &cum[now..], &mut memo);
        }
    }
    for x in query {
        if x > 1000 {
            let pos = cum.partition_point(|&c| x > 1000 + c);
            if pos == cum.len() {
                println!("{}", x - cum[n]);
            } else {
                println!("{}", memo[pos][x - cum[pos]]);
            }
        } else {
            println!("{}", memo[0][x])
        }
    }
}
