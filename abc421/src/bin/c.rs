use proconio::*;

fn main() {
    input! {_: usize, s: marker::Bytes}

    let mut res = usize::MAX;
    for c in [b'A', b'B'] {
        let mut cnt = 0usize;
        let mut diff = 0;
        for (i, b) in s.iter().enumerate() {
            if *b == c {
                diff += (cnt * 2).abs_diff(i);
                cnt += 1;
            }
        }

        res = res.min(diff);
    }

    println!("{res}")
}
