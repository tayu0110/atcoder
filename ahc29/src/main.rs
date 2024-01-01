use proconio::*;
use std::io::Write;

const T: usize = 1000;

fn select_and_insert_card(k: usize, pos: usize, card: &mut Vec<(usize, usize)>) {
    input_interactive!(new_card: [(usize, usize, usize); k]);
    for (t, w, p) in new_card {
        if p == 0 {
            card.insert(pos, (t, w));
            return;
        }
    }
}

fn main() {
    input_interactive!(n: usize, m: usize, k: usize, _: usize, mut card: [(usize, usize); n], mut project: [(usize, usize); m]);

    let mut money = 0;
    for _ in 0..T {
        if let Some(pos) = card.iter().copied().position(|c| c.0 == 4) {
            card.remove(pos);
            println!("{pos} 0");
            std::io::stdout().flush().unwrap();

            input_interactive!(p: [(usize, usize); m]);
            project = p;

            input_interactive!(new_money: usize);
            money = new_money;

            select_and_insert_card(k, pos, &mut card);
            continue;
        }
    }
}
