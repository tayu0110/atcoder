use std::{
    io::{stdin, Read},
    time::Instant,
};

use rand::{thread_rng, Rng};

const N: usize = 100;
const L: usize = 500_000;

fn input() -> [u32; N] {
    let mut s = String::new();
    stdin().lock().read_to_string(&mut s).unwrap();
    let mut s = s.lines();
    s.next();
    let s = s.next().unwrap();
    let mut buf = [0; N];
    for (s, buf) in s.split_ascii_whitespace().zip(buf.iter_mut()) {
        *buf = s.parse().unwrap();
    }
    buf
}

fn optimize(t: &[u32], cnt: &[u32], mut answer: [[u8; 2]; N]) -> (u32, [[u8; 2]; N]) {
    let mut ans = answer
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, [u8; 2])>>();
    ans.sort_unstable_by_key(|a| cnt[a.0]);
    let mut nt = t.into_iter().cloned().enumerate().collect::<Vec<_>>();
    nt.sort_unstable_by_key(|t| t.1);
    let score = nt
        .iter()
        .map(|t| t.1)
        .zip(ans.iter().map(|a| a.0))
        .map(|(t, ai)| t.abs_diff(cnt[ai]))
        .sum::<u32>();

    let mut trans = [0; N];
    for i in 0..N {
        trans[ans[i].0] = nt[i].0;
        let ti = nt[i].0;
        answer[ti] = ans[i].1;
    }
    for i in 0..N {
        answer[i][0] = trans[answer[i][0] as usize] as u8;
        answer[i][1] = trans[answer[i][1] as usize] as u8;
    }
    (score, answer)
}

fn calc_score(t: &[u32], answer: &[[u8; 2]], cnt: &mut [u32]) -> u32 {
    let mut now = 0;
    for _ in 0..L {
        cnt[now] += 1;
        now = answer[now][(cnt[now] - 1) as usize & 1] as usize;
    }
    cnt.into_iter()
        .zip(t)
        .map(|(c, t)| t.abs_diff(*c))
        .sum::<u32>()
}

fn main() {
    let t = input();

    let mut cnt = [0; N];
    let mut answer = [[u8::MAX; 2]; N];
    let tm = Instant::now();
    let mut best = [[u8::MAX; 2]; N];
    for i in 0..N {
        best[i] = [((i + 1) % N) as u8; 2];
    }
    let mut best_score = {
        calc_score(&t, &best, &mut cnt);
        let (_, opt) = optimize(&t, &cnt, best);
        cnt.fill(0);
        best = opt;
        calc_score(&t, &opt, &mut cnt)
    };
    let mut rng = thread_rng();
    let mut max_diff = 0;
    while tm.elapsed().as_millis() < 1985 {
        cnt.fill(0);
        answer.fill_with(|| [rng.gen_range(0..N as u8), rng.gen_range(0..N as u8)]);
        let orig = calc_score(&t, &answer, &mut cnt);
        let (mut score, mut opt) = optimize(&t, &cnt, answer);
        if orig + 30 < score {
            score = orig;
            opt = answer;
        }
        if score < best_score + 30 {
            cnt.fill(0);
            let actual = calc_score(&t, &opt, &mut cnt);
            max_diff = max_diff.max(actual.abs_diff(score));
            if actual < best_score {
                best_score = actual;
                best = opt;
            }
        }
    }
    for [a, b] in best {
        println!("{a} {b}");
    }
}
