use proconio::*;

fn main() {
    input! {mut n: marker::Chars}

    let c = n.pop().unwrap();
    let mut res = String::new();
    for c in n {
        match c {
            '0' => res.push_str("nil"),
            '1' => res.push_str("un"),
            '2' => res.push_str("bi"),
            '3' => res.push_str("tri"),
            '4' => res.push_str("quad"),
            '5' => res.push_str("pent"),
            '6' => res.push_str("hex"),
            '7' => res.push_str("sept"),
            '8' => res.push_str("oct"),
            '9' => res.push_str("enn"),
            _ => unreachable!(),
        }
    }

    match c {
        '0' => res.push_str("nilium"),
        '1' => res.push_str("unium"),
        '2' => res.push_str("bium"),
        '3' => res.push_str("trium"),
        '4' => res.push_str("quadium"),
        '5' => res.push_str("pentium"),
        '6' => res.push_str("hexium"),
        '7' => res.push_str("septium"),
        '8' => res.push_str("octium"),
        '9' => res.push_str("ennium"),
        _ => unreachable!(),
    }

    let res = res.replace("ennnil", "ennil");
    let mut res = res.chars().collect::<Vec<_>>();
    res[0] = res[0].to_uppercase().next().unwrap();
    println!("{}", res.into_iter().collect::<String>());
}
