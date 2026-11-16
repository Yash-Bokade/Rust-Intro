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
struct Polygon {
    name: String,
    shape_type: Shape,
    area: f64,
    perimeter: f64,
}
impl Polygon {
    fn new(name: String, shape_type: Shape, area: f64, perimeter: f64) -> Self {
        Self {
            name,
            shape_type,
            area,
            perimeter,
        }
    }
}

fn main() {

    let p1 = Polygon::new(
        String::from("Square"),
        Shape::SQUARE(3.2),
        10.0,
        10.0,
    );
    let p2= Polygon::new(
        String::from("Rectangle"),
        Shape::RECTANGLE(3.2,4.5),
        10.0,
        10.0,
    );
    let p3= Polygon::new(
        String::from("Triangle"),
        Shape::TRIANGLE(3.,4.0,5.0),
        10.0,
        10.0,
    );
    let p4= Polygon::new(
        String::from("Circle"),
        Shape::CIRCLE {r:10.},
        10.0,
        10.0,
    );

    println!("{:#?}", p1);
    println!("{:#?}", p2);
    println!("{:#?}", p3);
    println!("{:#?}", p4);
    // Polygon {
    //     name: "Square",
    //     shape_type: SQUARE(
    //         3.2,
    //     ),
    //     area: 10.0,
    //     perimeter: 10.0,
    // }
    // Polygon {
    //     name: "Rectangle",
    //     shape_type: RECTANGLE(
    //         3.2,
    //         4.5,
    //     ),
    //     area: 10.0,
    //     perimeter: 10.0,
    // }
    // Polygon {
    //     name: "Triangle",
    //     shape_type: TRIANGLE(
    //         3.0,
    //         4.0,
    //         5.0,
    //     ),
    //     area: 10.0,
    //     perimeter: 10.0,
    // }
    // Polygon {
    //     name: "Circle",
    //     shape_type: CIRCLE {
    //         r: 10.0,
    //     },
    //     area: 10.0,
    //     perimeter: 10.0,
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