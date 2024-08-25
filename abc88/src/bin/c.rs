#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {mut c: [[i32; 3]; 3]}

    for v in [0, 1, 2, 0].windows(2) {
        let mut k = [0; 3];
        for j in 0..3 {
            k[j] = c[v[0]][j] - c[v[1]][j];
        }

        if !(k[0] == k[1] && k[1] == k[2] && k[2] == k[0]) {
            println!("No");
            std::process::exit(0);
        }
    }

    println!("Yes");
}
