use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {s: marker::Bytes}

        let n = s.len();
        let mut cnt = [0; 26];
        for &c in &s {
            cnt[(c - b'a') as usize] += 1;
        }

        let mut dp = vec![false; n / 2 + 1];
        let mut ch = vec![0u8; n / 2 + 1];
        dp[0] = true;
        ch[0] = u8::MAX;
        for i in 0..26 {
            for j in (0..n / 2 + 1).rev() {
                if dp[j] && j + cnt[i] < dp.len() && !dp[j + cnt[i]] {
                    dp[j + cnt[i]] = true;
                    ch[j + cnt[i]] = i as u8;
                }
            }
        }

        if !dp[n / 2] {
            println!("No");
            continue;
        }

        let mut ev = [false; 26];
        let mut now = n / 2;
        while now > 0 {
            ev[ch[now] as usize] = true;
            now -= cnt[ch[now] as usize];
        }
        assert_eq!(now, 0);
        assert_eq!(
            ev.iter()
                .enumerate()
                .filter_map(|(i, &b)| b.then_some(cnt[i]))
                .sum::<usize>(),
            n / 2
        );

        let mut ret = vec![0; n];
        let mut o = 0;
        let mut e = 1;
        for i in 0..26 {
            if ev[i] {
                for _ in 0..cnt[i] {
                    ret[e] = i as u8 + b'a';
                    e += 2;
                }
            } else {
                for _ in 0..cnt[i] {
                    ret[o] = i as u8 + b'a';
                    o += 2;
                }
            }
        }

        println!("Yes");
        println!("{}", ret.into_iter().map(|c| c as char).collect::<String>())
    }
}
