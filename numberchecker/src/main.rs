use std::io;
macro_rules! no_str {
    // checks if it is a string and converts to i32
    ($c:expr) => {
        match $c.parse::<i32>() {
            Ok(..) => true,
            Err(..) => false,
        }
    };
}
fn main() {
    let mut a_s = String::new();
    let mut b_s = String::new();
    println!("Hello please enter a number that I will check: ");
    io::stdin().read_line(&mut a_s).expect("failed to read");
    println!("And another one: ");
    io::stdin().read_line(&mut b_s).expect("failed to read");
    let a_st = a_s.trim();
    let b_st = b_s.trim();
    let _conf_a = no_str!(a_st);
    let _conf_b = no_str!(b_st);
    if _conf_a == false || _conf_b == false {
        println!("Invalid input")
    } else {
        let _a = a_st.parse::<i32>().unwrap();
        let _b = b_st.parse::<i32>().unwrap();
        compare(_a, _b);
    }
}
fn compare(_a: i32, _b: i32) -> bool {
    if _a > _b {
        println!("{} is more than {}", _a, _b);
    }
    if _a < _b {
        println!("{} is less than {}", _a, _b);
    }
    if _a == _b {
        println!("{} equals {}", _a, _b);
    }
    return true;
}
