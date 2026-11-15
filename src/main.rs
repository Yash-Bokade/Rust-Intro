use std::io;
use std::io::Write;
use rand::random_range;

fn main() {
    //  Slices types in Rust
    /*
    Slices let you reference a contiguous sequence of elements in a collection.
    A slice is a kind of reference, so it does not have ownership.
     */


    // slice is a reference and not Ownership so hopefully not that hard
    let msg= String::from("apple is red!");
    // println!("{}", first_word(&msg));

    let charsOfMsg = msg.as_bytes();
    for (i,&char) in charsOfMsg.iter().enumerate() {
        if(char==b' '){
            println!("{}",i); // prints the index of spaces
        }
    }


    let slices= &msg[3..];
    println!("{}",slices);
}
// fn first_word(s: &String) -> usize { //  this is the same function that was defined in the rust book that returns the index of the first word;
//     let bytes = s.as_bytes();
//
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//
//     s.len()
// }
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