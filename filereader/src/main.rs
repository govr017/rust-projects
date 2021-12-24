use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};
fn main() {
    let mut action = String::new();
    let mut path = String::new();
    let mut text = String::new();
    println!("Please enter your desired action: Read(r), Write(w)");
    io::stdin()
        .read_line(&mut action)
        .expect("Failed to read input");
    if action.to_string().trim() == "r" {
        println!("Enter the directory of the file:");
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read input");
        rd(&path.trim()).expect("Couldn't access the file stopping process");
    }
    if action.to_string().trim() == "w" {
        println!("Enter the directory where you want to put your file e.g - /mnt/i/Documents/dev/rust/text.txt or I:/Documents/dev/rust/projects/filereader/text.txt");
        io::stdin().read_line(&mut path).expect("Invalid path");
        println!("And now write down the text you want to put in:");
        io::stdin().read_line(&mut text).expect("Invalid text");
        wr(&path.trim(), &text).expect("Invalid input");
        println!("Ending process");
    } else {
        println!("Ending process");
    }
}
fn rd(n: &str) -> Result<(), Error> {
    let input = File::open(n)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }
    Ok(())
}
fn wr(n: &str, x: &str) -> Result<(), Error> {
    let mut output = File::create(n)?;
    write!(output, "{}", x)?;
    Ok(())
}
