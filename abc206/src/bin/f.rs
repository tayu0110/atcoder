#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn solve(l: usize, r: usize, p: &Vec<(usize, usize)>, memo: &mut Vec<Vec<i32>>) -> i32 {
    if l == r {
        memo[l][r] = 0;
        return 0;
    }
    if memo[l][r] != std::i32::MAX {
        return memo[l][r];
    }

    let mut set = std::collections::HashSet::new();
    for (nr, nl) in p {
        if l <= *nr && *nl <= r {
            let res = solve(l, *nr, p, memo) ^ solve(*nl, r, p, memo);
            set.insert(res);
        }
    }

    let mut res = 0;
    while set.contains(&res) {
        res += 1;
    }

    memo[l][r] = res;
    return res;
}

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize, p: [(usize, usize); n]}

        let mut memo = vec![vec![std::i32::MAX; 101]; 101];

        let res = solve(1, 100, &p, &mut memo);

        if res == 0 {
            println!("Bob");
        } else {
            println!("Alice");
        }
    }
}
