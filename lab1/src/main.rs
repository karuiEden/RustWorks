use std::io;

fn main() {
    let x: i32 = 0;
    let y: i32;
    let z: i32;
    z = check(x);
    y = obr(z);
    println!("Value of half inverted number is: {y}");
}

fn check(_ch: i32) -> i32 {
    let mut ch = String::new();
    io::stdin().read_line(&mut ch).expect("Fail");
    let ch: i32 = ch.trim().parse().expect("Fail");
    return ch;
}

fn obr(mut a: i32) -> i32 {
    let mut b: i32 = 1;
    let mut sum: i32 = 0;
    let mut step1: i32 = 10;
    let mut step2: i32 = 1;
    if a < 0 {
        b = -1;
        a *= b;
    }
    while a > 0 {
        sum = sum + a % 10 * step1;
        step1 *= 100;
        a /= 10;
        sum = sum + a % 10 * step2;
        step2 *= 100;
        a /= 10;
    }
    return sum * b;
}
