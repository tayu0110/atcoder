use proconio::*;

fn main() {
    input! {x: marker::Chars}

    if x == vec!['1'] {
        println!("0");
        return;
    }

    if x.len() == 1 {
        println!("1");
        return;
    }

    if x[0] == '1' && x[1..].iter().all(|c| *c == '0') {
        println!("{}", x.len() - 1);
    } else {
        println!("{}", x.len())
    }
}
