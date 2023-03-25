#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, a: [[usize; w]; h], r: usize, c: usize, b: [[usize; c]; r]};

    for i in 0..(1 << h) as usize {
        if i.count_ones() != r as u32 {
            continue;
        }
        for j in 0..(1 << w) as usize {
            if j.count_ones() != c as u32 {
                continue;
            }
            let mut t = vec![];
            for k in 0..h {
                if (i & (1 << k)) != 0 {
                    let mut buf = vec![];
                    for l in 0..w {
                        if (j & (1 << l)) != 0 {
                            buf.push(a[k][l]);
                        }
                    }
                    t.push(buf);
                }
            }
            let mut bad = false;
            for k in 0..r {
                for l in 0..c {
                    if t[k][l] != b[k][l] {
                        bad = true;
                        break;
                    }
                }
                if bad {
                    break;
                }
            }
            if !bad {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }
    println!("No");
}
