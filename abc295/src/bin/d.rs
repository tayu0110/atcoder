use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let n = s.len();
    let mut cum = vec![vec![0; 10]; n + 1];
    for i in 0..n {
        cum[i + 1][s[i] as usize - b'0' as usize] += 1;
        for j in 0..10 {
            cum[i + 1][j] += cum[i][j];
        }
    }

    let mut res = 0;
    let mut rem = vec![0usize; 1 << 10];
    for i in 0..n + 1 {
        let mut t = 0;
        for j in 0..10 {
            cum[i][j] %= 2;
            t |= cum[i][j] << j;
        }

        res += rem[t];
        rem[t] += 1;
    }

    println!("{}", res)
}
