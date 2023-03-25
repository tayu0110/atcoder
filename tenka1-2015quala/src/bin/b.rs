use proconio::input;

fn main() {
    input! {n: usize, p: [(String, String); n]}

    let f = |v: Vec<u32>| v[3] + v[2] * 1000 + v[1] * 60 * 1000 + v[0] * 60 * 60 * 1000;
    let p = p
        .into_iter()
        .map(|(s, e)| {
            let vs = s
                .split(&[':', '.'][..])
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            let ve = e
                .split(&[':', '.'][..])
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            (f(vs), f(ve))
        })
        .collect::<Vec<(u32, u32)>>();

    if p.iter().all(|(s, e)| s < e) {
        for _ in 0..n {
            println!("-1");
        }
        return;
    }

    let (mut sr, mut er) = (0, std::u32::MAX);
    for &(s, e) in &p {
        if s >= e {
            sr = sr.max(s);
            er = er.min(e);
        }
    }

    for (s, e) in p {
        if s >= e {
            println!("{}", e + 1000 - s);
        } else {
            if (sr - 1000 < s && s < er + 1000) || (sr - 1000 < e && e < er + 1000) {
                println!("-1");
            } else if s <= sr && er <= e {
                println!("{}", e + 1000 - s)
            } else {
                println!("{}", e - s)
            }
        }
    }
}
