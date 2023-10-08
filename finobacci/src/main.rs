use std::io;

fn main() {
    let mut a: i32;

    let mut b: i32;

    let mut c: i32;

    let mut d = String::new();

    io::stdin().read_line(&mut d).expect("Fail");

    let mut d: i32 = d.trim().parse().expect("Fail");

    a = 0;
    b = 1;
    if d == 1 {
        println!("Fibonacci number is: {a}");
    } else if d == 2 {
        println!("Fibonacci number is: {b}");
    } else if d > 2 {
        println!("Fibonacci number is: {a}");
        println!("Fibonacci number is: {b}");
        d -= 2;

        while d > 0 {
            c = a + b;
            a = b;
            b = c;
            d -= 1;
            println!("Fibonacci number is: {c}");
        }
    }
}
