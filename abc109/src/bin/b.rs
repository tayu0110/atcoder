#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, w: [Chars; n]}

    let mut set = std::collections::HashSet::new();
    let mut prev = 'a';
    for v in w {
        if set.is_empty() {
            prev = *v.last().unwrap();
            set.insert(v);
        } else {
            if v[0] != prev {
                println!("No");
                std::process::exit(0);
            }
            prev = *v.last().unwrap();
            if set.contains(&v) {
                println!("No");
                std::process::exit(0);
            }
            set.insert(v);
        }
    }

    println!("Yes");
}
