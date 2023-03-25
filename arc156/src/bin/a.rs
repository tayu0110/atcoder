use proconio::{input, marker::Chars};

fn main() {
    input! {t: usize}

    let mut res = vec![];
    for _ in 0..t {
        input! {n: usize, s: Chars}

        let mut cnt = vec![];
        for i in 0..s.len() {
            if s[i] == '1' {
                cnt.push(i);
            }
        }

        if cnt.len() % 2 == 1 {
            res.push(-1);
        } else if cnt.len() == 2 {
            if cnt[1] - cnt[0] >= 2 {
                res.push(1);
            } else {
                if n < 4 {
                    res.push(-1)
                } else if cnt[0] >= 2 || n - cnt[1] - 1 >= 2 {
                    res.push(2);
                } else {
                    res.push(3);
                }
            }
        } else {
            res.push(cnt.len() as i32 / 2);
        }
    }

    for res in res {
        println!("{}", res)
    }
}
