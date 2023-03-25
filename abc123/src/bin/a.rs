#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {t: [usize; 5], k: usize}

    for i in 0..5 {
        for j in i+1..5 {
            if t[j] - t[i] > k {
                println!(":(");
                std::process::exit(0);
            }
        }
    }

    println!("Yay!");
}
