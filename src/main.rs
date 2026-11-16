use std::io;
use std::io::Write;

// Methods for Structs In Rust

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

fn main() {
    let P1 = Polygon::new(String::from("P1"), None, 0.0, 0.0);
    let p2 = Polygon::new(String::from("P2"), Some(ShapeV2::SQUARE(5.0)), 25.0, 20.0);

    println!("{:#?}", P1);
    println!("{:#?}", p2);
    //  Polygon {
    //     name: "P1",
    //     shape_type: None,
    //     area: 0.0,
    //     perimeter: 0.0,
    // }
    // Polygon {
    //     name: "P2",
    //     shape_type: Some(
    //         SQUARE(
    //             5.0,
    //         ),
    //     ),
    //     area: 25.0,
    //     perimeter: 20.0,
    // }
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