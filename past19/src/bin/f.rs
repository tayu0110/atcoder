use std::{
    collections::HashMap,
    io::{stdin, Read},
    mem::ManuallyDrop,
};

fn main() {
    let mut buf = ManuallyDrop::new(String::new());
    stdin().lock().read_to_string(&mut buf).ok();
    let mut buffer = buf.as_str();
    let (n, rem) = buffer.split_once('\n').unwrap();
    let n = n.parse::<usize>().unwrap();
    let (x, rem) = rem.split_once(' ').unwrap();
    let (y, rem) = rem.split_once('\n').unwrap();
    buffer = rem;

    let mut map = ManuallyDrop::new(HashMap::new());
    map.insert(x, 0);
    map.insert(y, 1);
    let mut cnt = 2u32;
    let mut t = ManuallyDrop::new(vec![vec![]; 2 * n + 2]);
    let mut p = buffer.split_ascii_whitespace();
    while let Some(a) = p.next() {
        let a = *map.entry(a).or_insert_with(|| {
            let res = cnt;
            cnt += 1;
            res
        });
        let b = map.entry(p.next().unwrap()).or_insert_with(|| {
            let res = cnt;
            cnt += 1;
            res
        });
        t[a as usize].push(*b);
    }

    let mut reached = vec![false; cnt as usize];
    let mut nt = Vec::with_capacity(cnt as usize);
    nt.push(0);
    'b: while let Some(now) = nt.pop() {
        for &to in &t[now as usize] {
            if !reached[to as usize] {
                reached[to as usize] = true;
                if to == 1 {
                    break 'b;
                } else {
                    nt.push(to);
                }
            }
        }
    }

    if reached[1] {
        println!("Yes")
    } else {
        println!("No")
    }
}
