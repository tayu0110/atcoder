use proconio::*;

fn main() {
    input! {s: marker::Chars}
    let len = s.len();
    let m = len - 1;

    let mut res = 0;
    for i in 0..(1 << m) {
        let mut sum = 0;
        let mut now = s[0] as usize - b'0' as usize;
        for j in 0..m {
            if i & (1 << j) != 0 {
                now = now * 10 + s[j + 1] as usize - b'0' as usize;
            } else {
                sum += now;
                now = s[j + 1] as usize - b'0' as usize;
            }
        }
        sum += now;

        res += sum;
    }

    println!("{}", res);
}
