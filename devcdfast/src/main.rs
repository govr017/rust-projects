use std::io;
fn main() {
    let mut answer = String::new();
    println!("Choose your destination: dev,rust,projects(obs),projects-rust");
    io::stdin().read_line(&mut answer).expect("Failed to read input");
    if answer.to_string().trim() == "dev" {
        println!("cd /mnt/i/Documents/dev/");
    }
    else if answer.to_string().trim() == "rust" {
        println!("cd /mnt/i/Documents/dev/rust/");
    }
    else if answer.to_string().trim() == "projects" {
        println!("cd /mnt/i/Documents/dev/rust/projects/")
    }
    else if answer.to_string().trim() == "projects-rust" {
        println!("cd /mnt/i/Documents/rust-projects/")
    }
}
