use proconio::input;

fn main() {
    input! {n: usize, k: usize, d: i64, a: [i64; n]}

    let mut memo = vec![vec![std::i64::MIN; k + 1]; d as usize];
    memo[0][0] = 0;

    for a in a {
        for nk in (0..k).rev() {
            for nd in 0..d {
                if memo[nd as usize][nk] == std::i64::MIN {
                    continue;
                }

                let nnd = (nd + a) % d;
                memo[nnd as usize][nk + 1] =
                    std::cmp::max(memo[nnd as usize][nk + 1], memo[nd as usize][nk] + a);
            }
        }
    }

    let res = memo[0][k];
    if res == std::i64::MIN {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
