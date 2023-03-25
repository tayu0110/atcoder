use proconio::{input, marker::Chars};

fn main() {
    input! {_: usize, mut k: usize, s: Chars}

    let mut t = String::new();
    for c in s {
        if c == 'o' && k > 0 {
            t.push('o');
            k -= 1;
        } else {
            t.push('x');
        }
    }

    println!("{}", t)
}
