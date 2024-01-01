use proconio::*;

fn main() {
    input! {_: usize,  m: usize, s: marker::Bytes}

    let mut rem = m;
    let mut logo = 0;
    let mut res = 0;

    for b in s {
        match b {
            b'0' => {
                rem = m;
                logo = res;
            }
            b'1' => {
                if rem > 0 {
                    rem -= 1;
                } else if logo > 0 {
                    logo -= 1;
                } else {
                    res += 1;
                }
            }
            b'2' => {
                if logo > 0 {
                    logo -= 1;
                } else {
                    res += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{res}")
}
