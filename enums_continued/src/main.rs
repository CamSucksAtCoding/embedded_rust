use std::f32::consts::PI;

struct Rectangle {
    length: f32,
    width: f32,
    height: f32,
    x_coord: f32,
    y_coord: f32,
}
enum Shape {
    Circle {
        x_coord: f32,
        y_coord: f32,
        radius: f32,
    },
    Rectangle(Rectangle),
    Square {
        x_coord: f32,
        y_coord: f32,
        side: f32,
    },
}
impl Shape {
    fn new_circle(x_coord: f32, y_coord: f32, radius: f32) -> Shape {
        Shape::Circle {
            x_coord,
            y_coord,
            radius,
        }
    }
    fn new_square(x_coord: f32, y_coord: f32, side: f32) -> Shape {
        Shape::Square {
            x_coord,
            y_coord,
            side,
        }
    }

    fn area(self: &Shape) -> f32 {
        match self {
            Shape::Circle { radius, .. } => radius.powf(2.0) * PI,
            Shape::Rectangle(Rectangle { length, width, .. }) => length * width,
            Shape::Square { side, .. } => side.powf(2.0),
        }
    }
}
fn main() {
    let new_shape = Shape::new_circle(69.69, 420.420, 67.67);
    let area = Shape::area(&new_shape);
    let new_rectangle = Shape::Rectangle(Rectangle {
        length: (3.0),
        width: (2.0),
        height: (1.0),
        x_coord: (1.0),
        y_coord: (1.0),
    });
    let new_sqr = Shape::new_square(45.6, -110.1, 20.0);
    let area_sqr = Shape::area(&new_sqr);
    let area_rec = Shape::area(&new_rectangle);
    println!("Area of Circle: {area}");
    println!("Area of Rectangle: {area_rec}");
    println!("Area of Square: {area_sqr}");
}
