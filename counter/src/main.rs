use std::io;
fn main() {
    let mut input = String::new();
    let x = 0i32;
    let mut y = 0i32;
    clr_scr();
    println!("0");
    while x == 0 {
        io::stdin().read_line(&mut input).expect("Failed to read");
        y = y + 1;
        clr_scr();
        println!("{}", y);
    }
}
fn clr_scr() -> () {
    print!("{esc}c", esc = 27 as char);
}
