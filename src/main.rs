use std::io::{self, Write};

fn read_line_or_eof(txt: &str) -> Option<String> {
  print!("{}: ", txt);
  io::stdout().flush().expect("Flush error");

  let mut input = String::new();
  match io::stdin().read_line(&mut input).expect("Read error") {
    0 => None,
    _ => Some(input),
  }
}

fn read_num(txt: &str) -> Option<f64> {
  loop {
    let input = read_line_or_eof(txt)?;

    match input.trim().parse::<f64>() {
      Ok(num) => return Some(num),
      Err(_) => println!(" Enter a number"),
    }
  }
}

fn read_op(txt: &str) -> Option<char> {
  loop {
    let input = read_line_or_eof(txt)?;
    let input = input.trim();

    match input {
      "+" | "-" | "*" | "/" | "%" => return input.chars().next(),
      _ => println!(" Invalid operator: '{}'", input),
    }
  }
}

fn main() {
  let Some(num1) = read_num(" Number 1") else { return println!(); };
  let Some(op) = read_op(" Operator (+, -, *, /, %)") else { return println!(); };
  let Some(num2) = read_num(" Number 2") else { return println!(); };

  let result = match op {
    '+' => num1 + num2,
    '-' => num1 - num2,
    '*' => num1 * num2,
    '/' => {
      if num2 == 0.0 {
        println!(" Error: Division by 0");
        return;
      }
      num1 / num2
    },
    '%' => {
      if num2 == 0.0 {
        println!(" Error: Division by 0");
        return;
      }
      num1 % num2
    },
    _ => unreachable!(),
  };

  println!(" Result: {} {} {} = {}", num1, op, num2, result);
}
