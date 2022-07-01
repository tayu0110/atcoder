use proconio::input;
use proconio::source::line::LineSource;
use std::io::BufReader;

fn main() {
    let mut source = LineSource::new(BufReader::new(std::io::stdin()));
    input! {from &mut source, n: usize};

    let mut root = 3;
    let mut dist = vec![0; n+1];
    for i in 1..n+1 {
        if i == root {
            continue;
        }
        println!("? {} {}", root, i);
        input! {from &mut source, d: i32};
        if d < 0 {
            std::process::exit(-1);
        }
        dist[i] = d;
    }

    let mut target = 1usize;
    if dist[1] <= dist[2] {
        target = 2;
    }

    let mut flag = false;
    for (i, v) in dist.iter().enumerate() {
        if i <= 2 {
            continue;
        }
        if *v + 1 == dist[target] {
            println!("? {} {}", i, target);
            input! {from &mut source, d: i32};
            if d < 0 {
                std::process::exit(-1);
            }
            if d == 1 {
                root = i;
                flag = true;
                break;
            }
        }
    }

    if !flag {
        println!("! 1");
    } else {
        let to = if target == 2 { 1 } else { 2 };
        println!("? {} {}", root, to);
        input! {from &mut source, d: i32};
        if d < 0 {
            std::process::exit(-1);
        }
        println!("! {}", d + 1);
    }
}