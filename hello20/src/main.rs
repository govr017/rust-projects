use std::io;
use std::io::Write; // <--- bring flush() into scope

fn main() {
    print!("Hello enter a number 1-20: ");
    io::stdout().flush().unwrap();
    let mut input_text = String::new();
    let mut cal = 1i8;
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();

    match trimmed.parse::<i8>() {
        Ok(i) => {
            while cal < i + 1 {
                let _ttch = cal;
                if i > 20 {
                    println!("{} is more than 20", i);
                    break;
                }
                println!("{}", _ttch);
                cal = cal + 1;
            }
        }
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}
