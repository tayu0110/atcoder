use proconio::*;

fn main() {
    input! {x: usize, y: usize}

    let mut a = [0; 11];
    a[1] = x;
    a[2] = y;
    for i in 3..11 {
        let b = a[i - 1] + a[i - 2];
        a[i] = b
            .to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
    }

    println!("{}", a[10]);
}
