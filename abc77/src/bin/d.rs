use proconio::input;

fn rec(
    n: usize,
    rest: usize,
    rem: usize,
    ck: &mut Vec<Vec<bool>>,
    memo: &mut Vec<Vec<usize>>,
) -> usize {
    if rest == 0 && rem == 0 {
        return 1;
    }

    if memo[rest][rem] > 0 {
        return memo[rest][rem];
    }

    if ck[rest][rem] {
        memo[rest][rem] = 2;
        return 2;
    }

    ck[rest][rem] = true;
    for i in 0..10 {
        if i <= rest && rec(n, rest - i, (rem * 10 + i) % n, ck, memo) == 1 {
            memo[rest][rem] = 1;
            return 1;
        }
    }

    memo[rest][rem] = 2;
    2
}

fn main() {
    input! {n: usize};

    let max = {
        let mut max = 0;
        let mut now = n;
        while now > 0 {
            max += now % 10;
            now /= 10;
        }
        max
    };

    let mut memo = vec![vec![0; n]; max + 1];
    let mut ck = vec![vec![false; n]; max + 1];
    for res in 1..max {
        if rec(n, res, 0, &mut ck, &mut memo) == 1 {
            println!("{}", res);
            std::process::exit(0);
        }
    }

    println!("{}", max);
}
