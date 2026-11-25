/*
 * Enums: Typically used to represent a value that can be one of several possible variants.
 *
 * When creating an enum you are essentially creationg a new tyoe that can have one of several
 * possible variants. Each variant can be associated with its own set of data, which allows you
 * to represent complex data structures using a single enum.
 *
 * Enums can have methods associated with them just like structs aswell
 *
 */
struct Point {
    x: i32,
    y: i32,
}
enum CarStatus {
    MovingUp { speed: u32 },
    MovingDown { speed: u32 },
    NotMoving(Point),
}
enum GameState {
    Pause,
    Restart,
    End,
}
fn main() {
    let mut current_car_status = CarStatus::MovingUp { speed: (400) };
    current_car_status = CarStatus::NotMoving(Point { x: (0), y: (1) });
    match current_car_status {
        CarStatus::MovingDown { speed } => println!("The car is moving down at {speed}"),
        CarStatus::MovingUp { speed } => println!("The car is moving up at {speed}"),
        CarStatus::NotMoving(Point { x, y }) => {
            println!("The car is not moving and is stopped at point ({x}, {y})")
        }
    }
}
