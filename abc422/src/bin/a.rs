use proconio::*;

fn main() {
    input! {s: String}

    let (s, t) = s.split_once('-').unwrap();
    let (s, t) = (s.parse::<usize>().unwrap(), t.parse::<usize>().unwrap());

    if t == 8 {
        println!("{}-{}", s + 1, 1);
    } else {
        println!("{}-{}", s, t + 1)
    }
}
