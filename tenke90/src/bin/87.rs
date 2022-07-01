use proconio::input;

const INF: i64 = 111222333444555666;

fn warshall_floyd(m: i64, p: i64, a: &Vec<Vec<i64>>) -> usize {
    let mut dp = a.iter().map(|v| v.iter().map(|w| if *w < 0 { m } else { *w }).collect()).collect::<Vec<Vec<i64>>>();
    // eprintln!("dp: {:?}", dp);
    let n = a.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let tmp = dp[i][k] + dp[k][j];
                dp[i][j] = std::cmp::min(dp[i][j], tmp);
            }
        }
    }

    // eprintln!("dp: {:?}", dp);

    let mut res = 0;
    for (i, w) in dp.iter().enumerate() {
        for (j, v) in w.iter().enumerate() {
            if i <= j {
                break;
            }
            if *v <= p {
                // eprintln!("i: {}, j: {}, v: {}", i, j, v);
                res += 1;
            }
        }
    }
    // eprintln!("res: {}\n", res);

    res
}

fn main() {
    input! {n: usize, p: i64, k: usize, a: [[i64; n]; n]};

    {
        let lk = warshall_floyd(0, p, &a);
        let rk = warshall_floyd(INF, p, &a);

        eprintln!("lk: {}, rk: {}", lk, rk);

        if lk < k || rk > k {
            println!("0");
            std::process::exit(0);
        }
        if rk == k {
            println!("Infinity");
            std::process::exit(0);
        }
    }

    let l = {
        let mut l = 0;
        let mut r = INF;
        while r - l > 1 {
            let m = (r + l) / 2;
            let rk = warshall_floyd(m, p, &a);
            if rk <= k {
                r = m;
            } else {
                l = m;
            }
        }
        l
    };
    eprintln!("l: {}\n", l);

    let r = {
        let mut l = 0;
        let mut r = INF;
        while r - l > 1 {
            let m = (r + l) / 2;
            let rk = warshall_floyd(m, p, &a);
            if rk < k {
                r = m;
            } else {
                l = m;
            }
        }
        l
    };
    eprintln!("r: {}", r);

    // println!("{}", std::cmp::max(0, r - l));
    println!("{}", r - l);
}