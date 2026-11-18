/*
 * Statics
 *
 * Are global variables, different from const variables but still follow the same declaration format
 *
 * Lifetimes: Both mut and imm static vars have the 'static' lifetime, meaning the live for the entire duration of the program
 *
 */
static COUNTER: u32 = 0;
//IMPORTANT, mut statics are unsafe and must be placed in an unsafe block
static mut _CHEESEBURGERS_ATE_TODAY: i32 = -100;
fn main() {
    println!("{COUNTER}");
    //println!("{CHEESEBURGERS_ATE_TODAY}"); //Wont compile due to being unsafe
}
