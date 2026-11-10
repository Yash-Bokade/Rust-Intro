use std::io;
use std::io::Write;
use rand::random_range;

fn main() {
    //  Control Flow
    let mut random_no: i32= random_range(1..100);

    let mut check_data : i32;

    'input_for_guess: loop{

        check_data = inputInt("Enter guess: ")  as i32;

        if(check_data==random_no) {
            println!("You got it!!!");
            break;
        }
        else if check_data>random_no {
            println!("Guess is Lower");
        }
        else{
            println!("Guess is Higher");
        }

    }
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