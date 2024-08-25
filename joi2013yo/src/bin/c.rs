use proconio::*;

fn main() {
    input! {n: usize, t: String, s: [marker::Chars; n]}

    let mut res = 0;
    'b: for s in s {
        for i in 0..s.len() {
            for j in 1..=s.len() {
                let ns = s[i..].iter().step_by(j).collect::<String>();
                if ns.starts_with(&t) {
                    res += 1;
                    continue 'b;
                }
            }
        }
    }

    println!("{res}")
}
