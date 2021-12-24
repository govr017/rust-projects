use std::io;
fn main() {
    let mut fav = String::new();
    let mut age = String::new();
    let mut name = String::new();
    println!("Hello this is your personal card maker please enter in your age: ");
    io::stdin()
        .read_line(&mut age)
        .expect("failed to read");
    println!("Now let me ask you about your favourite thing in the world: ");
    io::stdin()
        .read_line(&mut fav)
        .expect("failed to read");
    println!("Oh yeah almost forgot to get your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("failed to read");
    card(age, fav, name);
    
}
fn card(age: String, fav: String, name: String) -> () {
    println!("CARD OF AWESOMENESS
           
    NAME: {}
    AGE: {}
    FAVOURITE THING: {}",name, age, fav);
}
