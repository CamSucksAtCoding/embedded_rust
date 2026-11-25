struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    //This is a constructor
    /*
     * A constructor is a function that creates an instance of a struct.
     * In rust the convention is naming the constructor new to signify it being a constructor
     */
    fn new_rectangle(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    fn area_calc(self) -> u32 {
        let area = self.height * self.width;
        println!("Area of the rectangle is {area}");
        area
    }
}

fn main() {
    //Manual way of creating a struct without constructor
    let rect = Rectangle {
        width: 4,
        height: 10,
    };
    //Creating an instance of struct with constructor
    let rect2 = Rectangle::new_rectangle(10, 5);
    rect.area_calc();
    rect2.area_calc();
}
