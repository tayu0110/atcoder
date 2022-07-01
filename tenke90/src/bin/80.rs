use proconio::input;

fn main() {
    input! {n: usize, d: usize, a: [usize; n]};

    let mut t = vec![];
    for i in 0..d {
        let mut tmp = 0;
        for (j, v) in a.iter().enumerate() {
            if (*v & (1usize << i)) != 0 {
                tmp |= 1usize << j;
            }
        }

        t.push(tmp);
    }

    let mut dp = vec![0; 1 << n];
    dp[0] = 1usize;

    for v in t {
        for now in (0..(1 << n)).rev() {
            if dp[now] == 0 {
                continue;
            }
            let to = now | v;
            dp[to] += dp[now];
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}