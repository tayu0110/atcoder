use proconio::*;

fn main() {
    input! {h: usize, _: usize, mut s: [marker::Chars; h], mut t: [marker::Chars; h]}
    s.iter_mut().for_each(|v| v.sort());
    t.iter_mut().for_each(|v| v.sort());

    if s == t {
        println!("Yes")
    } else {
        println!("No")
    }
}
