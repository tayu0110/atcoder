use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let s = s.into_iter().map(|s| s - b'0').collect::<Vec<_>>();
    let mut res = String::new();
    let mut now = vec![0; n];
    let mut cur = n;
    while cur > 0 {
        cur -= 1;

        if now[cur] == s[cur] {
            continue;
        }

        if now[cur] == 0 {
            for i in 0..=cur {
                assert_eq!(now[i], 0);
                now[i] = 1;
                res.push('A');
            }
        } else {
            for i in 0..=cur {
                assert_eq!(now[i], 1);
                now[i] = 0;
                res.push('B');
            }
        }
    }

    println!("{}", res.len());
    println!("{res}");
}
