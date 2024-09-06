use itertools::Itertools;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut res = vec![];
    'b: for _ in 0..t {
        input! {a: usize, b: usize, c: usize}

        let mut cnt = 0;
        for i in 0..60 {
            if a & (1 << i) == 0 {
                if b & (1 << i) != 0 || c & (1 << i) != 0 {
                    res.push("No");
                    continue 'b;
                }
            } else {
                if b & (1 << i) != 0 {
                    let k = 2 + ((c >> i) & 1);
                    if cnt > 0 && cnt != k {
                        res.push("No");
                        continue 'b;
                    }
                    cnt = k;
                }
            }
        }

        res.push("Yes");
    }
    println!("{}", res.iter().join("\n"))
}
