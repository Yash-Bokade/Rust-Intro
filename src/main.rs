use std::io;
use std::io::Write;

// Match Control Flow in Rust

#[derive(Debug)]
enum Shape{
    SQUARE (f32),
    RECTANGLE(f32,f32),
    TRIANGLE(f32,f32,f32),
    CIRCLE{r:f32}
}
#[derive(Debug)]
enum ShapeV2<T>{
    SQUARE (T),
    RECTANGLE(T,T),
    TRIANGLE(T,T,T),
    CIRCLE{r:T}
}

#[derive(Debug)]
struct Polygon {
    name: String,
    shape_type: Option<ShapeV2<f32>>,
    area: f64,
    perimeter: f64,
}
impl Polygon {
    fn new(name: String, shape_type: Option<ShapeV2<f32>>, area: f64, perimeter: f64) -> Self { // I was thinking of using generis from Object, but it would be complex i will do tht when they teach generics
        Self {
            name,
            shape_type,
            area,
            perimeter,
        }
    }
}
impl Polygon {
    fn calculate_area(&mut self)->f32{
        match self.shape_type {
            None=>{println!("No shape type setting to 0");
                self.area=0.0;
            },
            Some(ShapeV2::SQUARE(s))=> { self.area = (s * s) as f64; },
            Some(ShapeV2::RECTANGLE(l,w))=> { self.area= (l * w) as f64 },
            Some(ShapeV2::TRIANGLE(a,b,c))=>{self.area = ((a + b + c) / 2.0) as f64 },
            Some(ShapeV2::CIRCLE{r})=>{self.area= (3.14 * r * r) as f64 }
        }
        self.area as f32
    }
}

fn main() {
    let mut P1 = Polygon::new(String::from("P1"), None, 0.0, 0.0);
    let mut P2 = Polygon::new(String::from("P2"), Some(ShapeV2::SQUARE(5.0)), 25.0, 20.0);

    println!("{}",P1.calculate_area());
    println!("{}",P2.calculate_area());

    //Sorry guys i am leaving if let ... else because I think its easy and no need to do it here.
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