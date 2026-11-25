struct KrabbyPatty {
    price: f32,
    patties: i8,
    condiments: String,
}
/*
 * Methods: are functions that are associated to a specific struct
 * and you need to have self:, because it signifies the instance of the struct
 * on which the method is being called on. It operates like this keyword from java
 */
impl KrabbyPatty {
    fn cook_wait_time(self: &KrabbyPatty) -> i8 {
        let wait_time = 2 * self.patties;
        println!("Your Krabby Patty will be ready in {wait_time} minutes.");
        wait_time
    }
    fn update_price(&mut self) -> f32 {
        if self.patties > 2 {
            println!("Total: ${}", self.price * 4.0);
            self.price * 4.0
        } else {
            println!("Total: ${}", self.price);
            self.price
        }
    }
}
fn main() {
    let mut nasty_patty = KrabbyPatty {
        price: 12.00,
        patties: 4,
        condiments: String::from("Mustard and Ketchup"),
    };
    nasty_patty.cook_wait_time();
    nasty_patty.update_price();
}
