use proconio::*;
use string::RollingHash;

fn main() {
    input! {h: usize, w: usize, mut s: [String; h], t: [String; h]}

    let mut rs = vec![];
    for mut s in s {
        let u = s.clone();
        s.push_str(&u);
        rs.push(RollingHash::new(&s));
    }

    let rt = t
        .iter()
        .map(String::as_str)
        .map(RollingHash::new)
        .collect::<Vec<_>>();

    for j in 0..w {
        if (0..h).all(|i| rs[i].get(j..j + w) == rt[i].get(..)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
