use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    let mut a = 0;
    for s in s {
        if s == "A" {
            a += 1;
        } else {
            if a == 0 {
                println!("NO");
                return;
            }
            a -= 1;
        }
    }

    if a == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
