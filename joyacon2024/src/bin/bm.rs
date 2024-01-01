use proconio::*;

fn main() {
    input! {s: String}

    for i in (0..s.len()).rev() {
        if i % 2 != 0 {
            continue;
        }

        let s = &s[..i];
        let (s, t) = s.split_at(i / 2);
        if s == t {
            println!("{i}");
            return;
        }
    }
}
