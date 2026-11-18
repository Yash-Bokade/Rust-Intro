
#[derive(Debug)]
pub enum Shape{
    SQUARE (f32),
    RECTANGLE(f32,f32),
    TRIANGLE(f32,f32,f32),
    CIRCLE{r:f32}
}

#[derive(Debug)]
pub enum ShapeV2<T>{
    SQUARE (T),
    RECTANGLE(T,T),
    TRIANGLE(T,T,T),
    CIRCLE{r:T}
}

#[derive(Debug)]
pub struct Polygon {
    name: String,
    shape_type: Option<ShapeV2<f32>>,
    area: f64,
    perimeter: f64,
}
impl Polygon {
    pub fn new(name: String, shape_type: Option<ShapeV2<f32>>, area: f64, perimeter: f64) -> Self { // I was thinking of using generis from Object, but it would be complex i will do tht when they teach generics
        Self {
            name,
            shape_type,
            area,
            perimeter,
        }
    }
    pub fn calculate_area(&mut self)->f32{
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