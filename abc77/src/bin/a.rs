use proconio::*;

fn main() {
    input! {c: [marker::Chars; 2]}

    let mut d = c.clone();
    d.swap(0, 1);
    d.iter_mut().for_each(|d| d.reverse());

    if d == c {
        println!("YES")
    } else {
        println!("NO")
    }
}
