use proconio::*;

fn main() {
    input! {s: String, n: usize, t: [String; n]}

    let mut res = 0;
    for mut t in t {
        t = t.repeat(2);
        t.pop();

        res += t.contains(&s) as usize;
    }

    println!("{res}")
}
