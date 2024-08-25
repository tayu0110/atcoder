use proconio::input;

fn response(mut ans: i128) {
    let mut bit = 0;
    let mut cnt = 0i8;
    println!("{}", ans.count_ones());
    while ans > 0 {
        if (ans & 1) == 1 {
            if cnt == 0 {
                print!("{}", bit);
            } else {
                print!(" {}", bit);
            }
            cnt += 1;
        }
        let tr = std::cmp::max(1, ans.trailing_zeros());
        bit += tr;
        ans >>= tr;
    }
    println!();
}

const INF: i128 = -1;
const MAX: usize = 8890;
fn main() {
    input! {n: i8, q: i16, a: [i16; n], p: [(i8, i8); q]};

    let mut bad = [0i128; 90];
    for (x, y) in p {
        bad[x as usize] |= 1i128 << y;
        bad[y as usize] |= 1i128 << x;
    }

    let max = a.iter().fold(0, |sum, x| sum + *x) + 1;
    let mut dp = [INF; MAX];
    dp[0] = 0i128;
    for (i, v) in a.iter().enumerate() {
        let v = *v;
        let t = 1i128 << (i+1);
        for j in (0..max-v).rev() {
            if dp[j as usize] == INF || (bad[i+1] & dp[j as usize]) != 0 {
                continue;
            }
            if dp[(j+v) as usize] != INF {
                response(dp[(j+v) as usize]);
                response(dp[j as usize] | t);
                std::process::exit(0);
            }
            dp[(j+v) as usize] = dp[j as usize] | t;
        }
    }
}