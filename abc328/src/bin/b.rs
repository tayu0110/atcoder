use proconio::*;

fn main() {
    input! {n: usize, d: [usize; n]}

    let mut res = 0;
    for (i, d) in d.into_iter().enumerate() {
        for j in 1..=d {
            let s = format!("{}{}", i + 1, j);
            let mut s = s.chars().collect::<Vec<_>>();
            s.sort();
            s.dedup();
            if s.len() == 1 {
                res += 1;
            }
        }
    }

    println!("{res}")
}
