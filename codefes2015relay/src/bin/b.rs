use proconio::*;

const N: usize = 10;

fn main() {
    input! {p: [marker::Chars; N]}

    if p.iter()
        .map(|v| {
            v.iter()
                .fold(0, |s, v| (s << 1) | (*v == 'o') as usize)
        })
        .fold(0, |s, v| s | v)
        == (1 << N) - 1
    {
        println!("Yes")
    } else {
        println!("No")
    }
}
