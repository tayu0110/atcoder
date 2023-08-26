use itertools::Itertools;
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Normal};
use std::{collections::HashMap, io::Write};

const QUERY_MAX: usize = 10000;
const SAMPLE_COUNT: usize = 5;
const SQURE: [(i32, i32); 8] = [
    (0i32, 1i32),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn gen_f(sigma: f64) -> impl FnMut() -> f64 {
    let mut rng = thread_rng();
    let dist = Normal::new(0.0, sigma).unwrap();
    move || dist.sample_iter(&mut rng).take(SAMPLE_COUNT).sum::<f64>() / SAMPLE_COUNT as f64
}

fn calc_neighbor(x: usize, dx: i32, l: usize) -> usize {
    ((x as i32 + dx + l as i32) % l as i32) as usize
}

fn main() {
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));

    input!(l: usize, n: usize, _s: usize, p: [(usize, usize); n - 1]);
    let mut f = gen_f(_s as f64);
    let mut rng = thread_rng();

    let mut mapping = HashMap::new();
    let mut out = vec![vec![0; l]; l];
    let mut placed = vec![vec![false; l]; l];
    for (i, (y, x)) in p.into_iter().enumerate() {
        mapping.insert((y, x), i);
        out[y][x] = (1000 / n as i64) * i as i64;
        placed[y][x] = true;
    }

    for i in 0..100 {
        for _ in 0..500 {
            for i in 0..l {
                for j in 0..l {
                    if placed[i][j] {
                        continue;
                    }

                    let now = out[i][j];
                    let mut diff = 0;
                    for (dy, dx) in SQURE.iter().cloned() {
                        let y = calc_neighbor(i, dy, l);
                        let x = calc_neighbor(j, dx, l);
                        diff += out[y][x] - now;
                    }

                    out[i][j] += diff / SQURE.len() as i64;
                }
            }
        }

        for _ in 0..i * 100 {
            let y = rng.gen::<usize>() % l;
            let x = rng.gen::<usize>() % l;
            if placed[y][x] {
                continue;
            }

            let now = out[y][x];
            let mut diff = 0;
            for (dy, dx) in SQURE.iter().cloned() {
                let y = calc_neighbor(y, dy, l);
                let x = calc_neighbor(x, dx, l);
                diff += out[y][x] - now;
            }

            out[y][x] += diff / SQURE.len() as i64;
        }
    }

    let mut neisum = vec![0; n];
    for (&(y, x), &i) in mapping.iter() {
        neisum[i] = out[y][x].pow(3);
        for (dy, dx) in SQURE.iter().cloned() {
            let y = calc_neighbor(y, dy, l);
            let x = calc_neighbor(x, dx, l);
            neisum[i] += out[y][x].pow(3);
        }
        neisum[i] /= SQURE.len() as i64 + 1;
    }

    for v in &out {
        println!("{}", v.iter().join(" "));
        std::io::stdout().flush().unwrap();
    }

    let mut used = vec![false; n];
    let mut res = vec![std::i64::MAX; n];
    let mut rem = QUERY_MAX;
    let mut q = QUERY_MAX / n;
    for i in 0..n {
        let mut cnt = 0;
        let mut sum = 0;
        while cnt + (SQURE.len() + 1) < q {
            cnt += 1;
            println!("{} 0 0", i);
            std::io::stdout().flush().unwrap();
            input!(mut m: f64);
            m -= f();
            sum += (m.round() as i64).pow(3);
            for (dy, dx) in SQURE.iter().cloned() {
                cnt += 1;
                println!("{} {} {}", i, dy, dx);
                std::io::stdout().flush().unwrap();
                input!(mut m: f64);
                m -= f();
                sum += (m.round() as i64).pow(3);
            }

            let ave = sum / cnt as i64;
            if let Some(f) = neisum
                .iter()
                .position(|&v| (ave.max(v) - ave.min(v)) < 10000)
            {
                if used[f] {
                    continue;
                }
                res[i] = f as i64;
                used[f] = true;
                break;
            }
        }

        if res[i] == std::i64::MAX {
            let ave = sum / cnt as i64;
            let mut diff = std::i64::MAX;
            let mut idx = 0;
            for i in 0..n {
                if used[i] {
                    continue;
                }
                let d = ave.max(neisum[i]) - ave.min(neisum[i]);
                if diff > d {
                    diff = d;
                    idx = i;
                }
            }
            res[i] = idx as i64;
            used[idx] = true;
        }

        rem -= cnt;
        // eprintln!("i: {i}, n: {n}, rem: {rem}, cnt: {cnt}");
        if i < n - 1 {
            q = rem / (n - 1 - i);
        }
    }

    println!("-1 -1 -1");
    std::io::stdout().flush().unwrap();

    for e in res {
        println!("{}", e);
    }
    std::io::stdout().flush().unwrap();
}
