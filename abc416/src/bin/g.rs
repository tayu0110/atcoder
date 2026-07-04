use proconio::*;
use rustc_hash::FxHashMap;

#[allow(unused)]
struct TrieCore {
    c: char,
    // 0 - 25: 'a'..='z'
    // 26: $
    next: [usize; 27],
}

fn main() {
    input! {n: usize, k: usize, mut s: [String; n]}

    if k == 1 {
        s.sort_unstable();
        println!("{}", s[0]);
        return;
    }

    let mut trie = vec![TrieCore {
        c: '$',
        next: [usize::MAX; 27],
    }];
    for s in s {
        let mut now = 0;
        for c in s.chars() {
            let core = &trie[now];
            let index = c as usize - b'a' as usize;
            if core.next[index] < usize::MAX {
                now = core.next[index];
            } else {
                trie[now].next[index] = trie.len();
                trie.push(TrieCore {
                    c,
                    next: [usize::MAX; 27],
                });
                now = trie[now].next[index];
            }
        }
        trie[now].next[26] = 0;
    }

    let mut res = String::new();
    let mut buf = FxHashMap::default();
    let pass = trie[0].next;
    buf.insert(0, 1);
    'outer: loop {
        let mut next = usize::MAX;
        for (&key, &v) in &buf {
            if trie[key].next[26] < usize::MAX {
                if v == k {
                    break 'outer;
                }
                for i in 0..26 {
                    if pass[i] < usize::MAX {
                        next = next.min(i);
                        break;
                    }
                }
            }
            for i in 0..26 {
                if trie[key].next[i] < usize::MAX {
                    next = next.min(i);
                    break;
                }
            }
        }

        res.push((next as u8 + b'a') as char);
        let mut new = FxHashMap::default();
        for (k, v) in buf {
            let nt = &trie[k].next;
            if nt[next] == usize::MAX && nt[26] == usize::MAX {
                continue;
            }

            if nt[next] < usize::MAX {
                let entry = new.entry(nt[next]).or_insert(0);
                *entry = v.max(*entry);
            }
            if nt[26] < usize::MAX && pass[next] < usize::MAX {
                let entry = new.entry(pass[next]).or_insert(0);
                *entry = (v + 1).max(*entry);
            }
        }
        buf = new;
    }

    println!("{}", res);
}
