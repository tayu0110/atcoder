use proconio::*;

fn main() {
    input! {s: String}

    let n = s.len();
    let mut res = 0.0f64;
    for start in 0..n {
        for end in start + 1..=n {
            let s = &s[start..end];

            if s.len() < 3 {
                continue;
            }
            if !s.starts_with('t') || !s.ends_with("t") {
                continue;
            }

            let x = s.chars().filter(|&c| c == 't').count();
            res = res.max((x as f64 - 2.0) / (s.len() as f64 - 2.0));
        }
    }

    println!("{res}")
}
