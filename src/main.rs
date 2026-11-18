pub mod polygon;

use std::io;
use std::io::Write;
use crate::polygon::*;
fn main() {
    Polygon::new(String::from("Square"), Some(ShapeV2::SQUARE(4.0)), 16.0, 16.0);
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