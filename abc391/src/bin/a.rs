use proconio::*;

fn main() {
    input! {mut d: marker::Chars}
    for d in d.iter_mut() {
        *d = match *d {
            'N' => 'S',
            'W' => 'E',
            'E' => 'W',
            'S' => 'N',
            _ => 'A',
        };
    }
    println!("{}", d.iter().collect::<String>())
}
