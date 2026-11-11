use std::io;
use std::io::Write;
use rand::random_range;

fn main() {
    //  Ownership in Rust
    let name:String=String::from("Apple");
    /*
    let new_name = name;
    println!("{}",name); // Error borrowed  Element
    */
    let new_name=name.clone();
    println!("{}",name);
    println!("{}",new_name);

    let msg = "The Msg: ";
    let mut a = input(msg);
    println!("{}",a);
    println!("{}",msg);
    // Ownership remains because &str implements the Copy trait


    let mm:String =String::from("ABCDEF");
    println!("{}",mm);
    get_ownership(mm);
    // println!("{}",mm);// Returns an err (Ownership is no longer valid)
}

fn get_ownership(var:String){
    println!("ownership Ended here val[{}]",var);
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