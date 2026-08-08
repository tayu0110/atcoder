use std::io::Write;

use proconio::*;

fn main() {
    input_interactive! {n: usize}

    let mut ret = 0;
    let (mut l, mut r) = (1, 2);
    while l <= n {
        while r <= n {
            println!("? {l} {r}");
            std::io::stdout().flush().unwrap();

            input_interactive!(res: String);
            if res == "Yes" {
                r += 1;
            } else {
                break;
            }
        }
        ret += r - l - 1;

        l += 1;
        if r == l {
            r += 1;
        }
    }

    println!("! {ret}");
    std::io::stdout().flush().unwrap();
}
