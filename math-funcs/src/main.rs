// most of these likely exist. just doing for fun.

fn main() {
    assert_eq!(max(44, 33), 44);
    assert_eq!(min(33, 44), 33);
    assert_eq!(floor(5.6), 5);
    assert_eq!(round(43.67), 44);
    assert_eq!(even(1), false);
    assert_eq!(pow(25, 3), 15625);
}

fn max(x: i64, y: i64) -> i64 {
    let c = x - y;  
    // println!("{}", (c >> 31) & 1);
    let k = (c >> 31) & 1;  
    (x - k * c) as i64
}

fn min(x: i64, y: i64) -> i64 {
    if x < y { x } else { y }
}

fn floor(x: f64) -> i64 {
    (x - x % 1.0) as i64
}

fn ceil(x: f64) -> i64 {
    (x + (1.0 * (x % 1.0))) as i64  // inner brackets not needed, just easier to read like that.
}

fn round(x: f64) -> i64 {
    if x % 1.0 < 0.5 { floor(x) } else { ceil(x) }
}

fn even(x: i64) -> bool {
    if x & 0 == 1 { true } else { false }
}

fn pow(mut x: i64, mut y: i64) -> i64 {
    let mut final_ans = 1;

    while y > 0 {
        let last_bit = y & 1;

        if last_bit == 1 {
            final_ans *= x;
        } 

        x *= x;

        y = y >> 1;
    }
    final_ans
}
