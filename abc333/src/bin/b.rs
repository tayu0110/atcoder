use proconio::*;

fn main() {
    input! {p: marker::Chars, q: marker::Bytes}

    let mut s = p
        .into_iter()
        .map(|c| c as i8 - b'A' as i8)
        .collect::<Vec<_>>();
    let mut t = q
        .into_iter()
        .map(|c| c as i8 - b'A' as i8)
        .collect::<Vec<_>>();
    s.sort();
    t.sort();

    fn dist(s: i8, t: i8) -> i8 {
        if (t - s).abs() == 1 || (t + 1) % 5 == s || (s + 1) % 5 == t {
            1
        } else {
            2
        }
    }

    if dist(s[0], s[1]) == dist(t[0], t[1]) {
        println!("Yes")
    } else {
        println!("No")
    }
}
