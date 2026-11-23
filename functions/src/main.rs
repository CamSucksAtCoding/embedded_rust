fn convert_kg_to_grams(kg_value: f64) -> f64 {
    let result: f64 = kg_value * 1000.00;
    println!("{kg_value:.2}kg in grams: {result:.2}g");
    result
}
fn combine_strings(str1: &str, str2: &str) -> String {
    //only need to convert one &str to string or else it will throw an error, I need to look into that because that seems weird
    let combined_string: String = String::from(str1.to_string() + " " + str2);
    println!("{combined_string}");
    combined_string
}
//Comeback after learning liftime specifiers
// fn find_largest_string(str1: &str, str2: &str) -> &'a str {
//     if str1.len() > str2.len() {
//         println!("String 1 is larger");
//         str1
//     } else if str1.len() == str2.len() {
//         println!("The string are the same size");
//         str1
//     } else {
//         println!("String 2 is larger");
//         str2
//     }
// }
fn main() {
    convert_kg_to_grams(5.0);
    combine_strings("Hey", "Dude");
}
