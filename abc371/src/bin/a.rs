use std::cmp::Ordering;

use proconio::*;

fn main() {
    input! {s: [char; 3]}

    let mut matrix = [[0; 3]; 3];
    matrix[0][1] = s[0] as u8;
    matrix[1][0] = s[0] as u8 ^ b'<' ^ b'>';
    matrix[0][2] = s[1] as u8;
    matrix[2][0] = s[1] as u8 ^ b'<' ^ b'>';
    matrix[1][2] = s[2] as u8;
    matrix[2][1] = s[2] as u8 ^ b'<' ^ b'>';

    let mut buf = (0..3).collect::<Vec<_>>();
    buf.sort_by(|&l, &r| {
        if matrix[l][r] == b'<' {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    println!("{}", (buf[1] as u8 + b'A') as char)
}
