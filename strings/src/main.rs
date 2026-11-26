fn main() {
    let my_name: String = String::from("Malum Caedo"); // this is a String

    //this is a String type, which is used when the length of the string is unknown at compile time
    let mut showers_taken_this_year = String::from("I have taken 3 showers this year");
    println!("{showers_taken_this_year}... I'm tellling the truth");
    showers_taken_this_year.replace_range(12..13, "0");
    println!("Ok fine...{showers_taken_this_year}");

    //create a empty string variable
    let mut fav_color = String::new();
    fav_color.push_str("Grabber Blue");
    println!("My favorite color is {fav_color}");

    //Create a shallow copy(move/transfer ownership) of name string
    let fav_character = String::from(my_name);
    println!(
        "This is a shallow copy of my_name stored in fav_character. my_name = Invalid and fav_character = {fav_character}"
    ); //my_name is now invalid and creates an error because ownership of my_name is transfered to fav_character

    //Create a deep copy of fav_character
    let deep_copy = String::from(fav_character.clone());
    println!(
        "This is a deep copy of fav_character stored in deep_copy. fav_character = {fav_character} and deep_copy = {deep_copy}"
    ); //No error because I made a deep copy and didnt transfer ownership from the original
}
