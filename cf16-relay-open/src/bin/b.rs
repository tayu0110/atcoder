use proconio::*;

fn main() {
    input! {s: String}

    let t = s
        .chars()
        .rev()
        .map(|c| match c {
            'b' => 'd',
            'd' => 'b',
            'p' => 'q',
            'q' => 'p',
            c => c,
        })
        .collect::<String>();

    if s == t {
        println!("Yes")
    } else {
        println!("No")
    }
}
