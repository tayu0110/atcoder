use std::sync::Mutex;

use proconio::*;

const MASK: [usize; 25] = {
    let mut mask = [0; 25];
    let mut i = 0;
    while i < 25 {
        mask[i] = 0b11 << (i * 2);
        i += 1;
    }
    mask
};

// return true if `turn` wins.
// memo: 0: unknown, 1: win, 2: lose
fn solve(turn: usize, field: usize, cards: &[usize], memo: &mut [[u8; 1 << 24]]) -> bool {
    if memo[turn][field] != 0 {
        return memo[turn][field] == 1;
    }

    for i in 0..cards.len() {
        if (field >> (i * 2)) & 0b11 == turn {
            let nf = field | MASK[i];
            for j in 0..i {
                if cards[i] > cards[j] && field & MASK[j] == MASK[j] {
                    let nf = nf & !MASK[j] | (turn << (j * 2));
                    if !solve(1 - turn, nf, cards, memo) {
                        memo[turn][field] = 1;
                        return true;
                    }
                }
            }
            if !solve(1 - turn, nf, cards, memo) {
                memo[turn][field] = 1;
                return true;
            }
        }
    }
    memo[turn][field] = 2;
    false
}

static MEMO: Mutex<[[u8; 1 << 24]; 2]> = Mutex::new([[0; 1 << 24]; 2]);

fn main() {
    input! {n: usize, m: usize, l: usize, a: [usize; n], b: [usize; m], c: [usize; l]}

    let mut cards = a
        .into_iter()
        .map(|a| (a, 0))
        .chain(b.into_iter().map(|b| (b, 1)))
        .chain(c.into_iter().map(|c| (c, 3)))
        .collect::<Vec<_>>();
    cards.sort_unstable();
    // 0: takahashi, 1: aoki: 3: in the field
    let mut field = 0;
    for (i, &(_, t)) in cards.iter().enumerate() {
        field |= t << (i * 2);
    }
    let (cards, _) = cards
        .into_iter()
        .unzip::<usize, usize, Vec<usize>, Vec<_>>();

    let mut memo = MEMO.lock().unwrap();
    if solve(0, field, &cards, memo.as_mut_slice()) {
        println!("Takahashi")
    } else {
        println!("Aoki")
    }
}
