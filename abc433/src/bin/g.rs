use proconio::*;
// use rand::{thread_rng, Rng};
use string::SuffixArray;
// use ac_library::suffix_array;

fn dfs(now: usize, mut start: usize, end: usize, s: &[u8], sa: &SuffixArray) -> bool {
    if end - start == 1 {
        let index = sa[start] as usize;
        return s[index + now..].len() % 2 != 0;
    }

    while start < end {
        let index = sa[start] as usize;
        if s[index..].len() <= now {
            start += 1;
            continue;
        }

        let (mut l, mut r) = (start, end);
        while r - l > 1 {
            let m = (r + l) / 2;
            let i = sa[m] as usize;
            // eprintln!(
            //     "index: {index}, start: {start}, end: {end}, i: {i}, m: {m}, now: {now}, s[index..]: {}, s[i..]: {}",
            //     std::str::from_utf8(&s[index..]).unwrap(),
            //     std::str::from_utf8(&s[i..]).unwrap()
            // );
            if s[i..][now] > s[index..][now] {
                r = m;
            } else {
                l = m;
            }
        }

        let ret = dfs(now + 1, start, r, s, sa);
        if !ret {
            return true;
        }
        start = r;
    }
    false
}

fn main() {
    input! {t: usize}
    // let t = 10000000;

    // let mut rng = thread_rng();
    for _ in 0..t {
        // ucbpgpgfqtfbirqxkfggxfphesjbfbjhumooucrgcatspkprcogpqsdxlwasjqnbkbbozgajnbhhilexbifnqjbfbcad
        input! {s: String}
        // let len = rng.gen_range(1..100);
        // let s = (0..len)
        //     .map(|_| (rng.gen_range(0..26) + b'a') as char)
        //     .collect::<String>();
        // eprintln!("s: {s}");
        let sa = SuffixArray::new(&s);
        // let sa = suffix_array(&s);
        // eprintln!("sa: {:?}", sa.iter().collect::<Vec<_>>());
        // let mut nsa = (0..s.len() as u32).collect::<Vec<_>>();
        // nsa.sort_by_key(|&i| &s[i as usize..]);
        // eprintln!("nsa: {:?}", nsa);
        // assert!(sa.iter().copied().collect::<Vec<_>>() == nsa);
        if dfs(0, 0, s.len(), s.as_bytes(), &sa) {
            println!("Alice")
        } else {
            println!("Bob")
        }
    }
}
