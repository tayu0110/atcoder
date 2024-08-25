#[allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, mut p: [(usize, usize); m]}
    if m == 0 {
        println!("{}", ["Aoki", "Takahashi"][n % 2]);
        return;
    }

    p.push((0, std::usize::MAX));
    p.push((n+1, std::usize::MAX));
    p.sort();

    let mut g = 0;
    for v in p.windows(2) {
        if let [(px, py), (x, y)] = v {
            if py == &std::usize::MAX || y == &std::usize::MAX {
                g ^= *x - *px - 1;
            } else if py == y {
                g ^= 1;
            }
        }
    }

    if g == 0 {
        println!("Aoki");
    } else {
        println!("Takahashi");
    }
}
