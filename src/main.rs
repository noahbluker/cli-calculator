use std::io::{self, Write};

fn main()
{
    let number1  = read_number(" Number 1");
    let operator = read_operator(" Operator (+, -, *, /, %)");
    let number2  = read_number(" Number 2");

    let result = match operator {
        '+' => number1 + number2,
        '-' => number1 - number2,
        '*' => number1 * number2,
        '/' => {
            if number2 == 0.0 {
                println!(" ! Error : Division by 0");
                return;
            }
            number1 / number2
        },
        '%' => {
            if number2 == 0.0 {
                println!(" ! Error : Division by 0");
                return;
            }
            number1 % number2
        },
        _ => unreachable!(),
    };

    println!(" Result : {} {} {} = {}", number1, operator, number2, result);
}

fn read_number(txt: &str) -> f64 {
    loop {
        print!("{} : ", txt);
        io::stdout().flush().expect("Flush error");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read error");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!(" ! Enter a number"),
        }
    }
}

fn read_operator(txt: &str) -> char {
    loop {
        print!("{} : ", txt);
        io::stdout().flush().expect("Flush error");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Read error");

        let input = input.trim();

        match input {
            "+" | "-" | "*" | "/" | "%" => return input.chars().next().unwrap(),
            _ => println!(" ! Invalid operator : '{}'", input),
        }
    }
}