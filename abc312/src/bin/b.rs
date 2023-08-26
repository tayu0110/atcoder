use proconio::*;

fn main() {
    input! {n: usize, m: usize, s: [marker::Chars; n]}

    let mut res = vec![];
    for i in 0..n {
        for j in 0..m {
            if i + 9 > n || j + 9 > m {
                continue;
            }
            let mut t = vec![];
            for k in i..i + 9 {
                t.push(s[k][j..j + 9].to_vec());
            }

            let mut bad = false;
            for i in 0..3 {
                for j in 0..3 {
                    if t[i][j] != '#' {
                        bad = true;
                    }
                }
            }
            for i in 6..9 {
                for j in 6..9 {
                    if t[i][j] != '#' {
                        bad = true;
                    }
                }
            }
            for i in 0..4 {
                if t[3][i] == '#' {
                    bad = true;
                }
                if t[i][3] == '#' {
                    bad = true;
                }
            }
            for i in 5..9 {
                if t[5][i] == '#' {
                    bad = true;
                }
                if t[i][5] == '#' {
                    bad = true;
                }
            }
            if !bad {
                res.push((i + 1, j + 1));
            }
        }
    }

    res.sort();
    for (a, b) in res {
        println!("{} {}", a, b);
    }
}
