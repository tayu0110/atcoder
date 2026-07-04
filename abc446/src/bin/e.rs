use proconio::*;

fn solve(m: usize, a: usize, b: usize, x: usize, y: usize, memo: &mut [Vec<i8>]) -> i8 {
    if memo[x][y] >= 0 {
        return memo[x][y];
    }

    if memo[x][y] == -2 {
        return 0;
    }

    let z = (a * y + b * x) % m;
    if z == 0 {
        memo[x][y] = 1;
        return 1;
    }

    memo[x][y] = -2;
    memo[x][y] = solve(m, a, b, y, z, memo);
    memo[x][y]
}

fn main() {
    input! {m: usize, a: usize, b: usize}

    // -2: calculating, -1: undecided, 0: ok, 1: bad
    let mut memo = vec![vec![-1i8; m]; m];
    let mut ret = 0;
    for x in 1..m {
        for y in 1..m {
            if memo[x][y] == -1 {
                solve(m, a, b, x, y, &mut memo);
            }

            if memo[x][y] == 0 {
                ret += 1;
            }
        }
    }

    println!("{ret}")
}
