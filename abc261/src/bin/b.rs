#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, a: [Chars; n]};

    for i in 0..n {
        for j in i+1..n {
            let (l, r) = (a[i][j], a[j][i]);
            if l == 'D' && r == 'D' {
                continue;
            }
            if l == 'W' && r == 'L' {
                continue;
            }
            if l == 'L' && r == 'W' {
                continue;
            }
            println!("incorrect");
            std::process::exit(0);
        }
    }
    println!("correct");
}
