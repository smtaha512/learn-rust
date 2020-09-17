use std::io;

fn main() {
    println!("nth Fibonacci!");

    println!("Enter n:");

    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Failed to read n");

    let n: i32 = loop {
        let n: i32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Invalid input. ");
                continue;
            }
        };
        break n;
    };

    let mut fib = 0;
    let mut last_n = 0;
    let mut number = 0;
    while number < n {
        let last_fib = fib;
        fib = last_n + fib + if number < 1 { 1 } else { 0 };
        number = number + 1;
        last_n = last_fib;
    }

    println!("nth fibonacci is: {}", fib);
}
