use proconio::*;

fn main() {
    input! {n: usize}
    for i in 1..=n {
        let mut s = String::new();
        if i % 2 == 0 {
            s.push('a');
        }
        if i % 3 == 0 {
            s.push('b');
        }
        if i % 4 == 0 {
            s.push('c');
        }
        if i % 5 == 0 {
            s.push('d');
        }
        if i % 6 == 0 {
            s.push('e');
        }
        if s.is_empty() {
            s.push_str(i.to_string().as_str());
        }
        println!("{s}")
    }
}
