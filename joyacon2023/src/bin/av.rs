use proconio::input;

fn main() {
    input! {n: usize, p: [(String, String); n]}

    for (i, (s, t)) in p.iter().cloned().enumerate() {
        let (mut f, mut g) = (false, false);
        for (j, (p, q)) in p.iter().cloned().enumerate() {
            if i == j {
                continue;
            }

            if s == p || s == q {
                f = true;
            }

            if t == p || t == q {
                g = true;
            }
        }

        if f && g {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
