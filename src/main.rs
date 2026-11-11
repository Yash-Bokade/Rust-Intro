use std::io;
use std::io::Write;
use rand::random_range;

fn main() {
    //  References and Borrowing in Rust
    let name ="Hello, world!";
    println!("Hello, {}!", name);
    borrow_(name);
    println!("Hello, {}!", name); // no error because the referencing has not dropped the var from the memory
}

// fn dangle()->&String{
//     let str = String::from("hello");         This is not possible in rust because Rust compiler guarantees
//     &str                                     that the data will not go out of scope before the reference to that data
// }


fn borrow_(var:&str){
    println!("borrowing a var -> val[{}]",var);
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