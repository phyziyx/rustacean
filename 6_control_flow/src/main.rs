use std::io;

fn main() {
    // Generate the nth Fibonacci number.
    println!("Fibonacci generation:");

    println!("Fibonacci of 2: {}", fibonacci(2)); // 1
    println!("Fibonacci of 3: {}", fibonacci(3)); // 2
    println!("Fibonacci of 4: {}", fibonacci(4)); // 3
    println!("Fibonacci of 5: {}", fibonacci(5)); // 5
    println!("Fibonacci of 6: {}", fibonacci(6)); // 8

    // Convert temperatures between Fahrenheit and Celsius.
    println!("Temperature converter:");

    fn take_float_input() -> f32 {
        println!("Please insert your temperature value: ");

        let mut buf: String = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read an input");

        buf.trim()
            .parse::<f32>()
            .expect("Please insert a numeric value")
    }

    loop {
        println!("Choose temperature that you wish to convert to:");
        println!("1. Fahrenheit to Celsius.");
        println!("2. Celsius to Fahrenheit.");
        println!("Type  'exit',  'stop'  or  'quit'  to exit.");

        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read input.");

        let sanitised_buffer = buf.trim();
        match sanitised_buffer {
            "quit" | "stop" | "exit" => {
                break;
            }
            "1" => {
                let source = take_float_input();
                let target = (source - 32.0) * (5.0 / 9.0);
                println!("{}F converted to {}C", source, target)
            }
            "2" => {
                let source = take_float_input();
                let target = (source * (9.0 / 5.0)) + 32.0;
                println!("{}C converted to {}F", source, target)
            }
            _ => {
                println!("Invalid input specified!");
                continue;
            }
        }
    }
}

fn fibonacci(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;

    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    for _ in 2..=n {
        let next = a + b;

        a = b;
        b = next;
    }

    b
}
