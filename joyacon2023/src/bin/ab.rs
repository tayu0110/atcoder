use proconio::input;

fn main() {
    input! {a: usize, b: usize, c: usize, d: usize, e: usize, f: usize, x: usize}

    let tak = {
        let mut x = x;
        let mut res = 0;
        while x > 0 {
            res += std::cmp::min(a, x) * b;
            x = x.saturating_sub(a + c);
        }
        res
    };
    let aok = {
        let mut x = x;
        let mut res = 0;
        while x > 0 {
            res += std::cmp::min(d, x) * e;
            x = x.saturating_sub(d + f);
        }
        res
    };

    if tak > aok {
        println!("Takahashi")
    } else if tak < aok {
        println!("Aoki")
    } else {
        println!("Draw")
    }
}
