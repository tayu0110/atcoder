#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars};
    let set = s.into_iter().collect::<std::collections::HashSet<_>>();
    
    if set.len() == 1 {
        println!("Won");
    } else {
        println!("Lost");
    }
}
