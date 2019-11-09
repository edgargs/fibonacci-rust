use std::env;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let x = args[1].parse::<u128>().unwrap();
    println!("Value of parameter is: {}",x);
    
    //println!("{}", fibonacci(x));

    fn fib(up_to: u128) -> u128 {
        let mut a = 0;
        let mut b = 1;
        for i in 0..up_to {
            let addition: u128 = a + b;
            //println!("{}",i);
            a = b;
            b = addition;
        }
        b
    }
    println!("Result with  Rust: {}", fib(x));
}
