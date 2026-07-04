use proconio::*;

fn main() {
    input! {_: usize, t: marker::Bytes}

    let mut parity = [1, 0];
    let mut res = 0usize;
    let mut cnt = 0;
    for c in t {
        if c == b'0' {
            cnt += 1;
        }
        res += parity[cnt % 2];
        parity[cnt % 2] += 1;
    }

    println!("{}", res)
}
