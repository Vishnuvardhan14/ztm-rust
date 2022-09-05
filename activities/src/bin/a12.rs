// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color{
    Brown,
    Red
}
impl Color{
    fn print(&self){
        match self{
            Color::Brown => PRINTLN!("brown"),
            Color::Red=>println!("red"),
        }
    }
}

Struct Dimensions{
    width:f64,
    height:f64,
    depth:f64,
}
impl Dimensions{
    fn print(&self){
        println!("width:{:?}",self.width);
        println!("height:{:?}",self.height);
        println!("depth:{:?}",self.depth);
    }
}
Struct ShippingBox{
    color: Color,
    weight: f64,
    dimensions: Dimensions
}

impl ShippingBox{
    fn new(weight:f64, color: Color, dimensions: Dimensions) -> Self{
        Self{
            weight,
            color,
        }
    }
    fn print(&self){
        self.color.print();
    }
}
fn main(){
    
}