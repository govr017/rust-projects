use std::io;
use std::io::Write; // <--- bring flush() into scope

const I_MIN: i32 = 1;
const I_MAX: i32 = 20;
fn main() {
    let mut x: i32 = 1;
    print!("Hello enter a number {}-{}: ", I_MIN, I_MAX);
    io::stdout().flush().unwrap();
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(trimmedint) => while x == 1 {
            print_loop(trimmedint);
            x = 0;
        },
        Err(..) => println!("Invalid input. note: try using a whole number"),
    }


}

fn print_loop( i: i32) -> bool {
    // println!("*** print_loop({})", i);
    if i < I_MIN {
        println!("{} is less than {}", i, I_MIN);
        return false;
    } else if i > I_MAX {
        println!("{} is more than {}", i, I_MAX);
        return false;
    }

    let mut cal : i32 = 1;
    while cal < i+1 {
        let _ttch = cal;
        println!("{}",_ttch);
        cal = cal+1;
    }
    return true
}
