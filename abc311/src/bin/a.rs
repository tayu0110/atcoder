use proconio::*;

fn main() {
    input! {_: usize, s: marker::Chars}

    let mut f = [false; 3];
    for (i, c) in s.into_iter().enumerate() {
        match c {
            'A' => f[0] = true,
            'B' => f[1] = true,
            'C' => f[2] = true,
            _ => unreachable!(),
        }
        if f.iter().fold(true, std::ops::BitAnd::bitand) {
            println!("{}", i + 1);
            return;
        }
    }
}
