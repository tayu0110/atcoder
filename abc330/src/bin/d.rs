use proconio::*;

fn main() {
    input! {n: usize, s: [marker::Chars; n]}

    let mut r = vec![0; n];
    let mut c = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                r[j] += 1;
                c[i].push(j);
            }
        }
    }

    let mut res = 0;
    for v in c {
        let len = v.len();
        for c in v {
            res += (r[c] - 1) * (len - 1);
        }
    }

    println!("{}", res)
}
