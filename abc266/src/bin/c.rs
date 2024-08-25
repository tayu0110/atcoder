#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {a: [i32; 2], b: [i32; 2], c: [i32; 2], d: [i32; 2]};

    if in_area(&a, &b, &c, &d) || in_area(&a, &b, &d, &c) || in_area(&a, &d, &c, &b) || in_area(&d, &b, &c, &a) {
        println!("No");
    } else {
        println!("Yes");
    }
}

fn in_area(a: &[i32], b: &[i32], c: &[i32], p: &[i32]) -> bool {
    let (ax, ay) = (a[0], a[1]);
    let (bx, by) = (b[0], b[1]);
    let (cx, cy) = (c[0], c[1]);
    let (px, py) = (p[0], p[1]);
    let area = (-by * cx + ay * (-bx + cx) + ax * (by - cy) + bx * cy) as f64 / 2.0;
    let s = 1.0 / (area * 2.0) * (ay * cx - ax * cy + (cy - ay) * px + (ax - cx) * py) as f64;
    let t = 1.0 / (area * 2.0) * (ax * by - ay * bx + (ay - by) * px + (bx - ax) * py) as f64;
    0.0 < s && s < 1.0 && 0.0 < t && t < 1.0 && 0.0 < 1.0-s-t && 1.0-s-t < 1.0
}
//(ax, ay),(bx, by),(cx, cy)の三角形
//(px, py)が判定したい点
// double Area = 0.5 *(-by*cx + ay*(-bx + cx) + ax*(by - cy) + bx*cy);
// double s = 1/(2*Area)*(ay*cx - ax*cy + (cy - ay)*px + (ax - cx)*py);
// double t = 1/(2*Area)*(ax*by - ay*bx + (ay - by)*px + (bx - ax)*py);
 
// if((0 < s < 1) && (0 < t < 1)&&(0<1-s-t<1)){
//     return 1; //Inside Triangle
// }else{
//     return 0;
// }