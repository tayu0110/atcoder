use proconio::*;

const M: u32 = 998244353;

#[derive(Debug)]
struct TrieNode {
    ch: u8,
    depth: usize,
    // 0-25: a-z
    next: [usize; 26],
    bit: usize,
}

fn main() {
    input! {mut n: usize, l: usize, mut s: [String; n]}

    let mut remove = vec![false; n];
    for i in 0..n {
        for j in 0..n {
            if i != j && s[j].contains(&s[i]) {
                remove[i] = true;
                break;
            }
        }
    }
    for i in (0..n).rev() {
        if remove[i] {
            s.remove(i);
            n -= 1;
        }
    }

    let mut i = 0;
    while i < s.len() {
        let mut j = i;
        while j < s.len() {
            for k in 0..s[i].len() {
                if k > s[j].len() {
                    continue;
                }
                if s[j][..s[j].len() - k] == s[i][..k] {
                    s.push(s[j].clone());
                }
            }
            j += 1;
        }
        i += 1;
    }

    let mut trie = vec![TrieNode {
        ch: b'$',
        depth: 0,
        next: [usize::MAX; 26],
        bit: 0,
    }];
    for (i, s) in s.into_iter().enumerate() {
        let mut now = 0;
        let mut depth = 0;
        for s in s.bytes() {
            let c = (s - b'a') as usize;
            depth += 1;
            if trie[now].next[c] == usize::MAX {
                trie[now].next[c] = trie.len();
                trie.push(TrieNode {
                    ch: c as u8,
                    depth,
                    next: [usize::MAX; 26],
                    bit: 0,
                });
            }
            now = trie[now].next[c];
        }
        trie[now].bit |= 1 << i;
    }

    eprintln!("trie: {trie:?}");

    let mut dp = vec![vec![vec![0u32; 1 << n]; trie.len()]; l + 1];
    dp[0][0][0] = 1;
    for i in 0..l {
        for j in 0..trie.len() {
            for s in 0..1 << n {
                if dp[i][j][s] == 0 {
                    continue;
                }

                for next in 0..26 {
                    if trie[j].next[next] == usize::MAX {
                        if trie[0].next[next] == usize::MAX {
                            dp[i + 1][0][s] += dp[i][j][s];
                            dp[i + 1][0][s] %= M;
                        } else {
                            let nj = trie[0].next[next];
                            let ns = s | trie[nj].bit;
                            dp[i + 1][nj][ns] += dp[i][j][s];
                            dp[i + 1][nj][ns] %= M;
                        }
                    } else {
                        let nj = trie[j].next[next];
                        let ns = s | trie[nj].bit;
                        dp[i + 1][nj][ns] += dp[i][j][s];
                        dp[i + 1][nj][ns] %= M;
                    }
                }
            }
        }
    }

    eprintln!("dp: {dp:?}");

    let mut res = 0;
    for j in 0..trie.len() {
        eprintln!("j: {j}, dp: {}", dp[l][j][(1 << n) - 1]);
        res += dp[l][j][(1 << n) - 1];
        res %= M;
    }

    println!("{res}")
}
