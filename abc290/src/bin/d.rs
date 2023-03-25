use proconio::input;
// use rand::{thread_rng, Rng};

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y != 0 {
        x %= y;
        std::mem::swap(&mut x, &mut y);
    }
    x
}

// fn greedy(n: usize, d: usize, mut k: usize) -> usize {
//     let mut f = vec![false; n];
//     let mut now = 0;
//     k -= 1;
//     f[0] = true;
//     while k > 0 {
//         k -= 1;

//         now = (now + d) % n;
//         while f[now] {
//             now += 1;
//             now %= n;
//         }

//         f[now] = true;
//     }

//     now
// }

fn main() {
    input! {t: usize}

    // let mut rng = thread_rng();

    for _ in 0..t {
        input! {n: usize, d: usize, k: usize}
        // let (n, d): (usize, usize) = (rng.gen_range(1, 10), rng.gen_range(1, 10));
        // let k = rng.gen_range(1, n + 1);

        let g = gcd(n, d);

        let res = if g == 1 {
            d * (k - 1) % n
        } else {
            let t = n / g;
            let k = k - 1;

            let (div, rem) = (k / t, k % t);
            (d * rem) % n + div
        };

        // assert_eq!(res, greedy(n, d, k));

        println!("{}", res);
    }
}
