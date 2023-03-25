#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize};
    // for i in n-7..=n {
    //     println!("{}", i);
    // }
    println!("{}", n-7);
    println!("{}", n-6);
    println!("{}", n-5);
    println!("{}", n-4);
    println!("{}", n-3);
    println!("{}", n-2);
    println!("{}", n-1);
    println!("{}", n);
}
