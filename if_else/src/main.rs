/*
 * If condition {
 *
 * }else{
 *
 * }
 *
 * if condition{
 *
 * }else if condition{
 *
 * }else{
 *
 * }
 *
 * if condition {
 *
 * }let
 *
 *
 *
 */
fn main() {
    let answer: bool = true;
    let motorcycle_lover: bool = if answer { true } else { false };
    println!("Do you love motorcycles? Answer: {motorcycle_lover}");
}
