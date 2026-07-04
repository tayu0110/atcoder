use proconio::*;

fn main() {
    input! {n: usize, s: [String; n]}

    let mut login = false;
    let mut res = 0;
    for s in s {
        if s == "login" {
            login |= true;
        } else if s == "logout" {
            login &= false;
        } else if s == "public" {
            // no op
        } else if !login {
            res += 1;
        }
    }

    println!("{res}")
}
