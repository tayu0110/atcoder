use proconio::*;

fn main() {
    input! {x: [usize; 5]}
    for i in 1..=5 {
        if !x.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
