use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, _: usize, mut a: [marker::Chars; n]}

    a.sort_by(|w, v| {
        let mut a = w.clone();
        let mut b = v.clone();

        a.extend(v.iter());
        b.extend(w.iter());

        a.cmp(&b)
    });

    a.reverse();

    println!(
        "{}",
        a.iter().map(|v| v.iter().collect::<String>()).join("")
    )
}
