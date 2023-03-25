use proconio::input;

fn main() {
    input! {n: usize, k: usize}

    let mut now = 0;
    for i in 3..=3 * n {
        if now + (i - 2) * (i - 1) / 2 < k {
            now += (i - 2) * (i - 1) / 2;
            continue;
        }

        eprintln!("i: {}, now: {}", i, now);

        for j in 1..i {
            if now + i - j < k {
                now += i - j;
                continue;
            }

            for l in 1..i - j {
                if now + 1 < k {
                    now += 1;
                    continue;
                }

                eprintln!("{}", now);
                println!("{} {} {}", j, l, i - j - l);
                return;
            }
        }
    }

    eprintln!("{}", now);
}
