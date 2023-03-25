#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut s: String};

    let v = s.chars().collect::<Vec<_>>();
    let mut buf = String::new();
    for (i, c) in v.iter().enumerate() {
        if c == &'d' {
            buf.push(*c);
            continue;
        }

        for j in i..n {
            let mut t = buf.clone();
            for k in (i..=j).rev() {
                t.push((v[k] as u8 ^ b'd' ^ b'p') as char);
            }
            for k in j+1..n {
                t.push(v[k]);
            }

            if t < s {
                s = t;
            }
        }
        break;
    }

    println!("{}", s);
}
