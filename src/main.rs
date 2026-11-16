use std::io;
use std::io::Write;

// Methods for Structs In Rust
#[derive(Debug)]
struct Rect {
    width:u32,
    height:u32
}
impl Rect{
    // fn new(width:u32,height:u32) -> Rect{
    //     Rect{
    //         width,               the Constructor like thing but not a real constructor
    //         height
    //     }
    // }
    fn new(width:u32,height:u32)->Self{
        Self{
            width,                  // Can call this a Constructor
            height
        }
    }
    fn can_hold(&self,rect:&Rect) -> bool{
        if(rect.width <= self.width && rect.height <= self.height)
        {
            return true;
        }
        false
    }
    // fn new(rect:&Rect)->Rect{
    //     Rect{
    //         width:rect.width,        I think I cant overload in rust [there might be a way i think will see it ahead probably]
    //         height:rect.height
    //     }
    // }
}

// we can create multiple impl blocks
impl Rect{
    fn display(&self){
        println!("Width: {}, Height: {}",self.width,self.height);
    }
}

fn main() {
    let r1 = Rect::new(10,10);
    let r2 = Rect::new(20,10);
    let r3 = Rect::new(15,5);
    println!("{:#?}",r1.can_hold(&r2));
    println!("{:#?}",r2.can_hold(&r3));
    println!("{:#?}",r2.can_hold(&r1));


    r1.display();
    r2.display();
    r3.display();

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