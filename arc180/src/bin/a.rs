use proconio::*;

const M: usize = 1000000007;

fn main() {
    input! {_: usize, s: marker::Bytes}

    let mut t = vec![];
    let mut cnt = 1;
    for s in s.windows(2) {
        if s[0] == s[1] {
            t.push(cnt);
            cnt = 0;
        }
        cnt += 1;
    }
    t.push(cnt);

    let mut res = 1;
    for t in t {
        res *= (t + 1) / 2;
        res %= M;
    }

    println!("{res}");
}
