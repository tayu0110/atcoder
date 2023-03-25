use proconio::input;

fn main() {
    input! {h: usize, m: usize}

    for m in m..60 {
        let (a, b) = (h / 10, h % 10);
        let (c, d) = (m / 10, m % 10);

        let ac = a * 10 + c;
        let bd = b * 10 + d;

        if ac < 24 && bd < 60 {
            println!("{} {}", h, m);
            return;
        }
    }

    for h in h + 1..48 {
        let h = h % 24;
        for m in 0..60 {
            let (a, b) = (h / 10, h % 10);
            let (c, d) = (m / 10, m % 10);

            let ac = a * 10 + c;
            let bd = b * 10 + d;

            if ac < 24 && bd < 60 {
                println!("{} {}", h, m);
                return;
            }
        }
    }
}
