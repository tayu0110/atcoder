use proconio::*;

fn main() {
    input! {n: usize, s: marker::Bytes}

    let mut back = s.iter().filter(|&&s| s == b'1').count();
    let mut res = 0;
    {
        let mut next_fill = 0;
        for i in 0..n {
            if s[i] == b'1' {
                res += i - next_fill;
                next_fill += 1;
            }
        }
    }

    let mut min = res;
    let mut front = 0;
    for i in 0..n {
        if s[i] == b'1' {
            front += 1;
            back -= 1;
        } else {
            res += front;
            res -= back;
        }

        min = min.min(res);
    }

    println!("{min}")
}
