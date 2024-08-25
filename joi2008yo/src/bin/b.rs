use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut j = 0;
    let mut i = 0;
    for v in s.windows(3) {
        if v == &['J', 'O', 'I'] {
            j += 1;
        } else if v == &['I', 'O', 'I'] {
            i += 1;
        }
    }

    println!("{j}");
    println!("{i}");
}
