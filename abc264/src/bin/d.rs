#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};
    let mut map = std::collections::HashMap::new();
    for (i, c) in "atcoder".to_string().chars().enumerate() {
        map.insert(c, i);
    }
    let mut res = 0;
    for i in 0..s.len() {
        for j in 0..i {
            if map.get(&s[j]).unwrap() > map.get(&s[i]).unwrap() {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
