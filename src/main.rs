use std::io;
use std::io::Write;

fn main() {
    // 1. Create a mutable string buffer
    let mut input = input("What is your name:");
    println!("Hello, {input}!",);

    let mut i=inputInt("Age: ");
    println!("Wow, {i}!",);


}
fn input(msg: &str) -> String
{
    print!("{msg}");
    let mut input = String::new();

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    String::from(input.trim())
}

fn inputInt(msg: &str) -> i64
{
    print!("{msg}");
    let mut i = String::new();

    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut i).expect("Failed to read line");
    i.trim().parse().expect("Please type a valid number!")

}



// Learnt Functions and IO in rust