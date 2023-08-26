use proconio::*;

fn main() {
    input! {s: [marker::Chars; 3]}

    let mut index = vec![0; 3];
    let mut now = 0;
    loop {
        if s[now].len() == index[now] {
            println!("{}", (b'A' + now as u8) as char);
            return;
        }

        let next = s[now][index[now]] as usize - b'a' as usize;
        index[now] += 1;
        now = next;
    }
}
