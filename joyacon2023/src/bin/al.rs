use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {s: [Chars; 10]}

    let (mut a, mut b, mut c, mut d) = (20, 0, 20, 0);
    for i in 0..10 {
        for j in 0..10 {
            if s[i][j] == '#' {
                a = std::cmp::min(a, i);
                b = std::cmp::max(b, i);
                c = std::cmp::min(c, j);
                d = std::cmp::max(d, j);
            }
        }
    }

    println!("{} {}", a + 1, b + 1);
    println!("{} {}", c + 1, d + 1);
}
