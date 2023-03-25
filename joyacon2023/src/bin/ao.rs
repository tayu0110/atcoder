use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {n: usize, m: usize, s: [Chars; n]}

    let mut v = vec![];
    for s in s {
        let mut k = 0;
        for (j, c) in s.into_iter().enumerate() {
            if c == 'o' {
                k |= 1 << j;
            }
        }
        v.push(k);
    }

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            if v[i] | v[j] == (1 << m) - 1 {
                res += 1;
            }
        }
    }

    println!("{}", res);
}
