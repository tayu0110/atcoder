#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: String};

    let (yy, mm) = s.split_at(2);
    let (yy, mm) = (yy.parse::<usize>().unwrap(), mm.parse::<usize>().unwrap());

    if 0 < yy && yy <= 12 && 0 < mm && mm <= 12 {
        println!("AMBIGUOUS");
    } else if 0 < yy && yy <= 12 {
        println!("MMYY");
    } else if 0 < mm && mm <= 12 {
        println!("YYMM");
    } else {
        println!("NA");
    }
}
