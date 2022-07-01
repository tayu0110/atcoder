use proconio::input;
use proconio::marker::Chars;

fn dfs(s: &mut [char], memo: &mut Vec<Vec<i64>>) -> i64 {
    if s.len() == 0 {
        return 0;
    }

    let mut res = 0;
    for i in 0..s.len() {
        while s[i] != 'a' {
            let c = s[i];
            let nc = c as u8 - 'a' as u8;
            s[i] = (c as u8- 1) as char;
            if memo[nc as usize][i] >= 0 {
                res += 1 + memo[nc as usize][i];
                continue;
            }

            for j in 0..i {
                s[j] = ((s[j] as u8 - 'a' as u8 + 1) % 3 + 'a' as u8) as char;
            }
            memo[nc as usize][i] = dfs(&mut s[0..i], memo);
            res += 1 + memo[nc as usize][i];
        }
    }

    res
}

fn main() {
    input! {n: usize, s: Chars};

    let mut s = s;
    let mut memo = vec![vec![-1; n]; 3];
    
    let res = dfs(&mut s[0..n], &mut memo);

    println!("{}", res);
}